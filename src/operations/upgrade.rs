use crate::args::UpgradeArgs;
use crate::builder::pacman::{PacmanColor, PacmanQueryBuilder};
use crate::internal::commands::ShellCommand;
use crate::internal::detect;
use crate::internal::error::SilentUnwrap;
use crate::internal::exit_code::AppExitCode;
use crate::internal::rpc::rpcinfo;
use crate::operations::aur_install::aur_install;
use crate::{prompt, Options};

/// Upgrades all installed packages
#[tracing::instrument(level = "trace")]
pub async fn upgrade(args: UpgradeArgs, options: Options) {
    if args.repo {
        upgrade_repo(options).await;
    }
    if args.aur {
        upgrade_aur(options).await;
    }
    if !args.aur && !args.repo {
        upgrade_repo(options).await;
        upgrade_aur(options).await;
    }
}

#[tracing::instrument(level = "trace")]
async fn upgrade_repo(options: Options) {
    let noconfirm = options.noconfirm;

    let mut pacman_args = vec!["-Syu"];
    if noconfirm {
        pacman_args.push("--noconfirm");
    }

    tracing::debug!("Upgrading repo packages");

    let pacman_result = ShellCommand::pacman()
        .elevated()
        .args(pacman_args)
        .wait()
        .await
        .silent_unwrap(AppExitCode::PacmanError);

    if pacman_result.success() {
        tracing::info!("Successfully upgraded repo packages");
    } else {
        let continue_upgrading = prompt!(default no,
            "Failed to upgrade repo packages, continue to upgrading AUR packages?",
        );
        if !continue_upgrading {
            tracing::info!("Exiting");
            std::process::exit(AppExitCode::PacmanError as i32);
        }
    }
}

#[tracing::instrument(level = "trace")]
async fn upgrade_aur(options: Options) {
    tracing::debug!("Upgrading AUR packages");

    let non_native_pkgs = PacmanQueryBuilder::foreign()
        .color(PacmanColor::Never)
        .query_with_output()
        .await
        .silent_unwrap(AppExitCode::PacmanError);

    tracing::debug!("aur packages: {non_native_pkgs:?}");
    let mut aur_upgrades = vec![];

    for pkg in non_native_pkgs {
        let remote_package = rpcinfo(&pkg.name)
            .await
            .silent_unwrap(AppExitCode::RpcError);

        if let Some(remote_package) = remote_package {
            if remote_package.metadata.version != pkg.version {
                tracing::debug!(
                    "local version: {}, remote version: {}",
                    pkg.version,
                    remote_package.metadata.version
                );
                aur_upgrades.push(pkg.name);
            }
        } else {
            tracing::warn!("Could not find the remote package for {}", pkg.name);
        }
    }

    if !aur_upgrades.is_empty() {
        let options = Options {
            upgrade: true,
            ..options
        };
        aur_install(aur_upgrades, options).await;
    } else {
        tracing::info!("No upgrades available for installed AUR packages");
    }

    tracing::info!("Scanning for .pacnew files post-upgrade");
    detect().await;
}
