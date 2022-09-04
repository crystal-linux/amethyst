use async_recursion::async_recursion;
use aur_rpc::PackageInfo;
use crossterm::style::Stylize;
use futures::future;
use indicatif::ProgressBar;

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::process::{ChildStderr, ChildStdout};
use tokio::task;

use crate::builder::git::{GitCloneBuilder, GitPullBuilder};
use crate::builder::makepkg::MakePkgBuilder;
use crate::builder::pacman::{PacmanInstallBuilder, PacmanUninstallBuilder};
use crate::builder::pager::PagerBuilder;
use crate::internal::dependencies::DependencyInformation;
use crate::internal::error::{AppError, AppResult, SilentUnwrap};
use crate::internal::exit_code::AppExitCode;
use crate::internal::utils::{get_cache_dir, wrap_text};
use crate::logging::get_logger;
use crate::logging::output::{print_aur_package_list, print_dependency_list};
use crate::logging::piped_stdio::StdioReader;
use crate::{cancelled, crash, multi_select, numeric, prompt, Options};

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

impl From<&PackageInfo> for BuildContext {
    fn from(p: &PackageInfo) -> Self {
        Self::from(p.to_owned())
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
        pb.finish_with_message("Couldn't find all packages".red().to_string());
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

    pb.finish_with_message("All packages found".green().to_string());
    get_logger().reset_output_type();

    if print_aur_package_list(&package_info.iter().collect::<Vec<_>>()).await
        && !options.noconfirm
        && !prompt!(default yes, "Some packages are already installed. Continue anyway?")
    {
        cancelled!();
    }

    let pb = get_logger().new_progress_spinner();
    pb.set_message("Fetching package information");

    let dependencies = future::try_join_all(
        package_info
            .iter()
            .map(|pkg| async { DependencyInformation::for_package(pkg).await }),
    )
    .await
    .silent_unwrap(AppExitCode::RpcError);
    pb.finish_and_clear();
    get_logger().reset_output_type();

    print_dependency_list(&dependencies).await;

    if !options.noconfirm
        && !prompt!(default yes, "Do you want to install these packages and package dependencies?")
    {
        cancelled!();
    }

    tracing::info!("Downloading sources");
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

    if !options.noconfirm {
        let to_review = multi_select!(&packages, "Select packages to review");
        for pkg in to_review.into_iter().filter_map(|i| packages.get(i)) {
            review_pkgbuild(pkg).await.unwrap();
        }
        if !prompt!(default yes, "Do you still want to install those packages?") {
            cancelled!();
        }
    }

    let aur_dependencies: Vec<&PackageInfo> = dependencies
        .iter()
        .flat_map(DependencyInformation::all_aur_depends)
        .collect();

    let repo_dependencies: HashSet<&str> = dependencies
        .iter()
        .flat_map(DependencyInformation::all_repo_depends)
        .collect();

    if !repo_dependencies.is_empty() {
        tracing::info!("Installing repo dependencies");
        PacmanInstallBuilder::default()
            .as_deps(true)
            .packages(repo_dependencies)
            .no_confirm(options.noconfirm)
            .install()
            .await
            .silent_unwrap(AppExitCode::PacmanError);
    }

    if !aur_dependencies.is_empty() {
        tracing::info!(
            "Installing {} from the aur",
            numeric!(aur_dependencies.len(), "package"["s"])
        );
        let batches = create_dependency_batches(aur_dependencies);
        tracing::debug!("aur install batches: {batches:?}");

        for batch in batches {
            install_aur_deps(batch, options.noconfirm).await.unwrap();
        }
    }

    tracing::info!("Installing {}", numeric!(contexts.len(), "package"["s"]));

    if let Err(e) = build_and_install(
        contexts,
        MakePkgBuilder::default(),
        PacmanInstallBuilder::default().no_confirm(options.noconfirm),
    )
    .await
    {
        handle_build_error(e)
            .await
            .silent_unwrap(AppExitCode::MakePkgError);
    }
    let make_depends = dependencies
        .iter()
        .flat_map(DependencyInformation::make_depends)
        .collect::<Vec<_>>();
    if !make_depends.is_empty()
        && !options.noconfirm
        && prompt!(default yes, "Do you want to remove the installed make dependencies?")
    {
        PacmanUninstallBuilder::default()
            .packages(make_depends)
            .no_confirm(true)
            .uninstall()
            .await
            .silent_unwrap(AppExitCode::PacmanError);
    }
    tracing::info!("Done!");
}

#[tracing::instrument(level = "trace")]
async fn install_aur_deps(deps: Vec<&PackageInfo>, no_confirm: bool) -> AppResult<()> {
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
        PacmanInstallBuilder::default()
            .no_confirm(no_confirm)
            .as_deps(true),
    )
    .await?;
    get_logger().reset_output_type();

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
    let results = future::join_all(
        ctxs.into_iter()
            .map(|ctx| build_package(ctx, make_opts.clone())),
    )
    .await;
    get_logger().reset_output_type();
    let mut ctxs = Vec::new();
    for result in results {
        match result {
            Ok(ctx) => ctxs.push(ctx),
            Err(e) => handle_build_error(e).await?,
        }
    }

    tracing::info!("Built {}", numeric!(ctxs.len(), "package"["s"]));
    tracing::info!("Installing packages");

    install_packages(ctxs, install_opts).await?;

    Ok(())
}

#[tracing::instrument(level = "trace", skip_all)]
async fn download_aur_source(mut ctx: BuildContext) -> AppResult<BuildContext> {
    let pb = get_logger().new_progress_spinner();
    let pkg_name = &ctx.package.metadata.name;
    let base_pkg = &ctx.package.metadata.package_base;
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
        let repository_url = format!("{aur_url}/{base_pkg}");
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

#[tracing::instrument(level = "trace")]
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
        .force(true)
        .spawn()?;

    let stderr = child.stderr.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    let handle = task::spawn({
        let pb = pb.clone();
        let pkg_name = pkg_name.clone();
        async move { show_and_log_stdio(stdout, stderr, pb, pkg_name).await }
    });

    let exit_status = child.wait().await?;
    handle.abort();

    if !exit_status.success() {
        pb.finish_with_message(format!(
            "{}: {}",
            pkg_name.clone().bold(),
            "Build failed!".red(),
        ));
        return Err(AppError::BuildError {
            pkg_name: pkg_name.to_owned(),
        });
    }

    let mut packages = MakePkgBuilder::package_list(build_path).await?;
    let match_version = ctx
        .package
        .metadata
        .version
        .rsplit_once('_')
        .map(|v| v.0)
        .unwrap_or(&ctx.package.metadata.version);
    let match_name = format!("{pkg_name}-{match_version}");
    tracing::debug!("Match name {match_name}");
    packages.retain(|name| {
        name.file_name()
            .and_then(|n| n.to_str())
            .unwrap()
            .starts_with(&match_name)
    });
    tracing::debug!("Archives: {packages:?}");
    pb.finish_with_message(format!("{}: {}", pkg_name.clone().bold(), "Built!".green()));
    ctx.step = BuildStep::Install(PackageArchives(packages));

    Ok(ctx)
}

#[tracing::instrument(level = "trace")]
async fn install_packages(
    mut ctxs: Vec<BuildContext>,
    install_opts: PacmanInstallBuilder,
) -> AppResult<Vec<BuildContext>> {
    let mut packages = Vec::new();

    for ctx in &mut ctxs {
        packages.append(&mut ctx.packages()?.clone());
        ctx.step = BuildStep::Done;
    }

    install_opts.files(packages).needed(false).install().await?;

    Ok(ctxs)
}

#[tracing::instrument(level = "trace")]
fn create_dependency_batches(deps: Vec<&PackageInfo>) -> Vec<Vec<&PackageInfo>> {
    let mut deps: HashMap<String, &PackageInfo> = deps
        .into_iter()
        .map(|d| (d.metadata.name.clone(), d))
        .collect();
    let mut batches = Vec::new();
    let mut relaxed = false;

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

            if (!contains_dep || relaxed) && contains_make_dep {
                deps.remove(&key);
                current_batch.insert(key, info);
                if relaxed {
                    break;
                }
            }
        }

        if current_batch.is_empty() {
            relaxed = true;
        } else {
            batches.push(current_batch.into_iter().map(|(_, v)| v).collect());
            relaxed = false;
        }
    }

    batches
}

#[tracing::instrument(level = "trace")]
async fn show_and_log_stdio(
    stdout: ChildStdout,
    stderr: ChildStderr,
    pb: Arc<ProgressBar>,
    package_name: String,
) -> AppResult<()> {
    let mut reader = StdioReader::new(stdout, stderr);
    let out_file = get_cache_dir().join(format!("{package_name}-build.log"));
    let mut out_writer = BufWriter::new(
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(out_file)
            .await?,
    );

    while let Ok(line) = reader.read_line().await {
        let _ = out_writer.write(line.as_bytes()).await?;
        let _ = out_writer.write(&[b'\n']).await?;
        tracing::trace!("{package_name}: {line}");
        let line = format!("{}: {}", package_name.clone().bold(), line);
        let lines = wrap_text(line);
        let line = lines.into_iter().next().unwrap();
        pb.set_message(line);
    }
    out_writer.flush().await?;

    Ok(())
}

#[tracing::instrument(level = "trace", skip_all)]
async fn handle_build_error<E: Into<AppError>>(err: E) -> AppResult<()> {
    get_logger().reset_output_type();
    let err = err.into();

    match err {
        AppError::BuildError { pkg_name } => {
            tracing::error!("Failed to build package {pkg_name}!");
            let log_path = get_cache_dir().join(format!("{pkg_name}-build.log"));
            review_build_log(&log_path).await?;

            Ok(())
        }
        e => Err(e),
    }
}

#[tracing::instrument(level = "trace")]
async fn review_build_log(log_file: &Path) -> AppResult<()> {
    if prompt!(default yes, "Do you want to review the build log?") {
        PagerBuilder::default().path(log_file).open().await?;
    }

    Ok(())
}

#[tracing::instrument(level = "trace")]
async fn review_pkgbuild(package: &str) -> AppResult<()> {
    let pkgbuild_path = get_cache_dir().join(package).join("PKGBUILD");
    PagerBuilder::default().path(pkgbuild_path).open().await?;

    Ok(())
}
