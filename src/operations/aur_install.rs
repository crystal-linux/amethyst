use async_recursion::async_recursion;
use aur_rpc::PackageInfo;
use crossterm::style::Stylize;
use futures::future;
use indicatif::ProgressBar;
use std::collections::{HashMap, HashSet};
use std::mem;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncReadExt, BufReader};
use tokio::task::JoinHandle;

use crate::builder::git::{GitCloneBuilder, GitPullBuilder};
use crate::builder::makepkg::MakePkgBuilder;
use crate::builder::pacman::PacmanInstallBuilder;
use crate::internal::dependencies::DependencyInformation;
use crate::internal::error::{AppError, AppResult, SilentUnwrap};
use crate::internal::exit_code::AppExitCode;
use crate::internal::utils::{get_cache_dir, wrap_text};
use crate::logging::get_logger;
use crate::logging::output::{print_aur_package_list, print_dependency_list};
use crate::{cancelled, crash, prompt, Options};

#[derive(Debug)]
pub struct BuildContext {
    pub package: PackageInfo,
    pub step: BuildStep,
}

#[derive(Debug)]
pub enum BuildStep {
    Download,
    Build(BuildPath),
    Install(PackageArchives),
    Done,
}

#[derive(Debug)]
pub struct BuildPath(pub PathBuf);

#[derive(Debug)]
pub struct PackageArchives(pub Vec<PathBuf>);

impl From<PackageInfo> for BuildContext {
    fn from(package: PackageInfo) -> Self {
        Self {
            package,
            step: BuildStep::Download,
        }
    }
}

impl BuildContext {
    pub fn build_path(&self) -> AppResult<&Path> {
        if let BuildStep::Build(path) = &self.step {
            Ok(&path.0)
        } else {
            Err(AppError::BuildStepViolation)
        }
    }

    pub fn packages(&self) -> AppResult<&Vec<PathBuf>> {
        if let BuildStep::Install(pkgs) = &self.step {
            Ok(&pkgs.0)
        } else {
            Err(AppError::BuildStepViolation)
        }
    }
}

/// Installs a given list of packages from the aur
#[tracing::instrument(level = "trace")]
#[async_recursion]
pub async fn aur_install(packages: Vec<String>, options: Options) {
    tracing::debug!("Installing from AUR: {:?}", &packages);

    let pb = get_logger().new_progress_spinner();
    pb.set_message("Fetching package information");

    let package_info = aur_rpc::info(&packages)
        .await
        .map_err(AppError::from)
        .silent_unwrap(AppExitCode::RpcError);

    tracing::debug!("package info = {package_info:?}");

    if package_info.len() != packages.len() {
        let mut not_found = packages.clone();
        package_info
            .iter()
            .for_each(|pkg| not_found.retain(|p| pkg.metadata.name != *p));
        crash!(
            AppExitCode::MissingDeps,
            "Could not find the package: {}",
            not_found.join(",").italic(),
        );
    }

    get_logger().reset_output_type();

    pb.finish_with_message("Found all packages in the aur");
    print_aur_package_list(&package_info);

    if !options.noconfirm
        && !prompt!(default yes, "Do you want to install those packages from the AUR?")
    {
        cancelled!();
    }

    tracing::info!("Downloading aur packages");
    get_logger().new_multi_progress();

    let dependencies = future::try_join_all(package_info.iter().map(|pkg| async {
        get_logger().new_progress_spinner().set_message(format!(
            "{}: Fetching dependencies",
            pkg.metadata.name.clone().bold()
        ));
        DependencyInformation::for_package(pkg).await
    }))
    .await
    .silent_unwrap(AppExitCode::RpcError);

    if !print_dependency_list(&dependencies) && !options.noconfirm {
        get_logger().print_newline();
        if !prompt!(default yes, "Do you want to install those dependencies?") {
            cancelled!();
        }
    }
    get_logger().new_multi_progress();

    let contexts = future::try_join_all(
        package_info
            .into_iter()
            .map(BuildContext::from)
            .map(download_aur_source),
    )
    .await
    .unwrap();

    get_logger().reset_output_type();
    tracing::info!("All sources are ready.");

    let aur_dependencies: Vec<PackageInfo> = dependencies
        .iter()
        .flat_map(|d| {
            let mut deps = d.make_depends.aur.clone();
            deps.append(&mut d.depends.aur.clone());

            deps
        })
        .collect();

    let repo_dependencies: HashSet<String> = dependencies
        .iter()
        .flat_map(|d| {
            let mut repo_deps = d.make_depends.repo.clone();
            repo_deps.append(&mut d.depends.repo.clone());

            repo_deps
        })
        .collect();

    if !repo_dependencies.is_empty() {
        tracing::info!("Installing repo dependencies");
        PacmanInstallBuilder::default()
            .as_deps(true)
            .no_confirm(true)
            .packages(repo_dependencies)
            .install()
            .await
            .silent_unwrap(AppExitCode::PacmanError);
    }

    if !aur_dependencies.is_empty() {
        tracing::info!(
            "Installing {} dependencies from the aur",
            aur_dependencies.len()
        );
        let batches = create_dependency_batches(aur_dependencies);
        tracing::debug!("aur install batches: {batches:?}");

        for batch in batches {
            install_aur_deps(batch).await.unwrap();
        }
    }

    tracing::info!("Installing {} packages", contexts.len());

    build_and_install(
        contexts,
        MakePkgBuilder::default(),
        PacmanInstallBuilder::default().no_confirm(true),
    )
    .await
    .silent_unwrap(AppExitCode::MakePkgError);
    tracing::info!("Done!");
}

async fn install_aur_deps(deps: Vec<PackageInfo>) -> AppResult<()> {
    get_logger().new_multi_progress();

    let dep_contexts = future::try_join_all(
        deps.into_iter()
            .map(BuildContext::from)
            .map(download_aur_source),
    )
    .await?;

    get_logger().reset_output_type();

    build_and_install(
        dep_contexts,
        MakePkgBuilder::default().as_deps(true),
        PacmanInstallBuilder::default().as_deps(true),
    )
    .await?;

    Ok(())
}

#[tracing::instrument(level = "trace")]
async fn build_and_install(
    ctxs: Vec<BuildContext>,
    make_opts: MakePkgBuilder,
    install_opts: PacmanInstallBuilder,
) -> AppResult<()> {
    tracing::info!("Building packages");
    get_logger().new_multi_progress();
    let ctxs = future::try_join_all(
        ctxs.into_iter()
            .map(|ctx| build_package(ctx, make_opts.clone())),
    )
    .await
    .silent_unwrap(AppExitCode::MakePkgError);
    get_logger().reset_output_type();

    tracing::info!("Built {} packages", ctxs.len());
    tracing::info!("Installing packages...");

    install_packages(ctxs, install_opts).await?;

    Ok(())
}

#[tracing::instrument(level = "trace", skip_all)]
async fn download_aur_source(mut ctx: BuildContext) -> AppResult<BuildContext> {
    let pb = get_logger().new_progress_spinner();
    let pkg_name = &ctx.package.metadata.name;
    pb.set_message(format!("{}: Downloading sources", pkg_name.clone().bold()));

    let cache_dir = get_cache_dir();
    let pkg_dir = cache_dir.join(&pkg_name);

    if pkg_dir.exists() {
        pb.set_message(format!(
            "{}: Pulling latest changes",
            pkg_name.clone().bold()
        ));
        GitPullBuilder::default().directory(&pkg_dir).pull().await?;
    } else {
        let aur_url = crate::internal::rpc::URL;
        let repository_url = format!("{aur_url}/{pkg_name}");
        pb.set_message(format!(
            "{}: Cloning aur repository",
            pkg_name.clone().bold()
        ));

        GitCloneBuilder::default()
            .url(repository_url)
            .directory(&pkg_dir)
            .clone()
            .await?;

        pb.set_message(format!(
            "{}: Downloading and extracting files",
            pkg_name.clone().bold()
        ));

        MakePkgBuilder::default()
            .directory(&pkg_dir)
            .no_build(true)
            .no_deps(true)
            .no_prepare(true)
            .skip_pgp(true)
            .run()
            .await?;
    }
    pb.finish_with_message(format!(
        "{}: {}",
        pkg_name.clone().bold(),
        "Downloaded!".green()
    ));
    ctx.step = BuildStep::Build(BuildPath(pkg_dir));

    Ok(ctx)
}

async fn build_package(
    mut ctx: BuildContext,
    make_opts: MakePkgBuilder,
) -> AppResult<BuildContext> {
    let pb = get_logger().new_progress_spinner();
    let pkg_name = &ctx.package.metadata.name;
    let build_path = ctx.build_path()?;
    pb.set_message(format!("{}: Building Package", pkg_name.clone().bold()));

    let mut child = make_opts
        .directory(build_path)
        .clean(true)
        .no_deps(true)
        .skip_pgp(true)
        .needed(true)
        .spawn()?;

    let stderr = child.stderr.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    let h1 = show_stdio_on_pb(stdout, pb.clone(), {
        let pkg_name = pkg_name.clone();
        move |s| format!("{}: {s}", pkg_name.clone().bold())
    });
    let h2 = show_stdio_on_pb(stderr, pb.clone(), {
        let pkg_name = pkg_name.clone();
        move |s| format!("{}: {s}", pkg_name.clone().bold())
    });

    loop {
        if let Some(exit_code) = child.try_wait()? {
            h1.abort();
            h2.abort();

            if !exit_code.success() {
                pb.set_message(format!(
                    "{}: {}",
                    "Build failed!".red(),
                    pkg_name.clone().bold()
                ));
                return Err(AppError::from("Failed to build package"));
            } else {
                break;
            }
        }
    }

    let packages = MakePkgBuilder::package_list(build_path).await?;
    pb.finish_with_message(format!("{}: {}", pkg_name.clone().bold(), "Built!".green()));
    ctx.step = BuildStep::Install(PackageArchives(packages));

    Ok(ctx)
}

async fn install_packages(
    mut ctxs: Vec<BuildContext>,
    install_opts: PacmanInstallBuilder,
) -> AppResult<Vec<BuildContext>> {
    let mut packages = Vec::new();

    for ctx in &mut ctxs {
        packages.append(&mut ctx.packages()?.clone());
        ctx.step = BuildStep::Done;
    }

    install_opts.files(packages).install().await?;

    Ok(ctxs)
}

#[tracing::instrument(level = "trace")]
fn create_dependency_batches(deps: Vec<PackageInfo>) -> Vec<Vec<PackageInfo>> {
    let mut deps: HashMap<String, PackageInfo> = deps
        .into_iter()
        .map(|d| (d.metadata.name.clone(), d))
        .collect();
    let mut batches = Vec::new();

    while !deps.is_empty() {
        let mut current_batch = HashMap::new();

        for (key, info) in deps.clone() {
            let contains_make_dep = info
                .make_depends
                .iter()
                .any(|d| current_batch.contains_key(d) || deps.contains_key(d));

            let contains_dep = info
                .depends
                .iter()
                .any(|d| current_batch.contains_key(d) || deps.contains_key(d));

            if !contains_dep && !contains_make_dep {
                deps.remove(&key);
                current_batch.insert(key, info);
            }
        }

        batches.push(current_batch.into_iter().map(|(_, v)| v).collect());
    }

    batches
}

fn show_stdio_on_pb<
    R: AsyncRead + Unpin + Send + 'static,
    F: Fn(String) -> String + 'static + Send,
>(
    stdout: R,
    pb: Arc<ProgressBar>,
    fmt: F,
) -> JoinHandle<()> {
    tokio::task::spawn({
        async move {
            let mut stdout = BufReader::new(stdout);
            let mut line = String::new();

            while let Ok(ch) = stdout.read_u8().await {
                if ch == b'\n' {
                    let line = fmt(mem::take(&mut line));
                    let lines = wrap_text(line);
                    let line = lines.into_iter().next().unwrap();
                    pb.set_message(line);
                } else {
                    line.push(ch as char);
                }
            }
        }
    })
}
