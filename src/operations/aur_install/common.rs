use std::{collections::HashMap, path::Path, sync::Arc};

use aur_rpc::PackageInfo;
use crossterm::style::Stylize;
use futures::future;
use indicatif::ProgressBar;
use tokio::{
    fs::OpenOptions,
    io::{AsyncWriteExt, BufWriter},
    process::{ChildStderr, ChildStdout},
    task,
};

use crate::{
    builder::{
        git::{GitCloneBuilder, GitPullBuilder},
        makepkg::MakePkgBuilder,
        pacman::PacmanInstallBuilder,
        pager::PagerBuilder,
    },
    internal::{
        error::{AppError, AppResult},
        utils::{get_cache_dir, wrap_text},
    },
    logging::piped_stdio::StdioReader,
    multi_progress, normal_output, numeric,
    operations::PackageArchives,
    prompt, spinner,
};

use super::{BuildContext, BuildPath, BuildStep};

#[tracing::instrument(level = "trace", skip_all)]
pub async fn download_aur_source(mut ctx: BuildContext) -> AppResult<BuildContext> {
    let pkg_name = &ctx.package.metadata.name;
    let base_pkg = &ctx.package.metadata.package_base;
    let pb = spinner!("{}: Downloading sources", pkg_name.clone().bold());

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
pub fn create_dependency_batches(deps: Vec<&PackageInfo>) -> Vec<Vec<&PackageInfo>> {
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
pub async fn build_and_install(
    ctxs: Vec<BuildContext>,
    make_opts: MakePkgBuilder,
    install_opts: PacmanInstallBuilder,
) -> AppResult<()> {
    tracing::info!("Building packages");
    multi_progress!();
    let results = future::join_all(
        ctxs.into_iter()
            .map(|ctx| build_package(ctx, make_opts.clone())),
    )
    .await;
    normal_output!();
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

#[tracing::instrument(level = "trace")]
async fn build_package(
    mut ctx: BuildContext,
    make_opts: MakePkgBuilder,
) -> AppResult<BuildContext> {
    let pkg_name = &ctx.package.metadata.name;
    let build_path = ctx.build_path()?;
    let pb = spinner!("{}: Building Package", pkg_name.as_str().bold());

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
            pkg_name.as_str().bold(),
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
    normal_output!();
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
