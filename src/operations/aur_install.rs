use async_recursion::async_recursion;
use aur_rpc::PackageInfo;
use crossterm::style::Stylize;
use futures::future;
use indicatif::ProgressBar;
use std::env;
use std::env::set_current_dir;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use std::time::Duration;
use tokio::fs;

use crate::builder::git::{GitCloneBuilder, GitPullBuilder};
use crate::internal::commands::ShellCommand;
use crate::internal::dependencies::DependencyInformation;
use crate::internal::error::{AppError, AppResult, SilentUnwrap};
use crate::internal::exit_code::AppExitCode;
use crate::internal::rpc::{self, rpcinfo};
use crate::internal::utils::get_cache_dir;
use crate::logging::get_logger;
use crate::{crash, internal::fs_utils::rmdir_recursive, prompt, Options};

/// Installs a given list of packages from the aur
#[tracing::instrument(level = "trace")]
#[async_recursion]
pub async fn aur_install(packages: Vec<String>, options: Options) {
    let noconfirm = options.noconfirm;

    tracing::debug!("Installing from AUR: {:?}", &packages);

    tracing::info!("Installing packages {} from the AUR", packages.join(", "));

    let pb = get_logger().new_progress_spinner();
    pb.set_message("Fetching package information");

    let package_info = aur_rpc::info(&packages)
        .await
        .map_err(AppError::from)
        .silent_unwrap(AppExitCode::RpcError);

    tracing::debug!("package info = {package_info:?}");
    tokio::time::sleep(Duration::from_secs(1)).await;

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

    pb.finish_with_message("Found all packages in the aur");

    get_logger().new_multi_progress();

    future::try_join_all(package_info.iter().map(download_aur_source))
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_secs(1)).await;

    let dependencies = future::try_join_all(package_info.iter().map(|pkg| async {
        get_logger()
            .new_progress_spinner()
            .set_message(format!("{}: Fetching dependencies", pkg.metadata.name));
        DependencyInformation::for_package(pkg).await
    }))
    .await
    .silent_unwrap(AppExitCode::RpcError);
    tokio::time::sleep(Duration::from_secs(1)).await;

    let aur_build_dependencies: Vec<PackageInfo> = dependencies
        .iter()
        .flat_map(|d| d.make_depends.aur.clone())
        .collect();

    let aur_dependencies: Vec<PackageInfo> = dependencies
        .iter()
        .flat_map(|d| d.depends.aur.clone())
        .collect();

    get_logger().reset_output_type();
    tracing::info!(
        "Installing {} build dependencies",
        aur_build_dependencies.len()
    );
    get_logger().new_multi_progress();

    future::try_join_all(aur_build_dependencies.iter().map(download_aur_source))
        .await
        .unwrap();
}

#[tracing::instrument(level = "trace", skip_all)]
async fn download_aur_source(info: &PackageInfo) -> AppResult<PathBuf> {
    let pb = get_logger().new_progress_spinner();
    let pkg_name = &info.metadata.name;
    pb.set_message(format!("{pkg_name}: Downloading sources"));

    let cache_dir = get_cache_dir();
    let pkg_dir = cache_dir.join(&pkg_name);
    tokio::time::sleep(Duration::from_secs(1)).await;

    if pkg_dir.exists() {
        pb.set_message(format!("{pkg_name}: Pulling latest changes {pkg_name}"));
        GitPullBuilder::default().directory(&pkg_dir).pull().await?;
        tokio::time::sleep(Duration::from_secs(1)).await;
    } else {
        let aur_url = rpc::URL;
        let repository_url = format!("{aur_url}/{pkg_name}");
        pb.set_message(format!("{pkg_name}: Cloning aur repository"));

        GitCloneBuilder::default()
            .url(repository_url)
            .directory(&pkg_dir)
            .clone()
            .await?;
        tokio::time::sleep(Duration::from_secs(1)).await;

        pb.set_message(format!("{pkg_name}: Downloading and extracting files"));
    }
    tokio::time::sleep(Duration::from_secs(1)).await;
    pb.finish_with_message(format!("{pkg_name} is ready to build"));

    Ok(pkg_dir)
}
