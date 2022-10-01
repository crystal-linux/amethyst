use crate::args::UpgradeArgs;
use crate::builder::pacman::{PacmanColor, PacmanQueryBuilder, PacmanUpgradeBuilder};
use crate::internal::detect;
use crate::internal::error::SilentUnwrap;
use crate::internal::exit_code::AppExitCode;
use crate::internal::rpc::rpcinfo;
use crate::operations::aur_install::aur_install;
use crate::{fl_error, fl_info, fl_warn, Options};

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
    let quiet = options.quiet;

    tracing::debug!("Upgrading repo packages");

    let result = PacmanUpgradeBuilder::default()
        .no_confirm(noconfirm)
        .quiet(quiet)
        .upgrade()
        .await;

    if result.is_err() {
        fl_error!("failed-upgrade-repo-pkgs");
        fl_info!("exiting");
        std::process::exit(AppExitCode::PacmanError as i32);
    } else {
        fl_info!("success-upgrade-repo-pkgs");
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
            fl_warn!("couldnt-find-remote-pkg", pkg = pkg.name);
        }
    }

    if !aur_upgrades.is_empty() {
        let options = Options {
            upgrade: true,
            ..options
        };
        aur_install(aur_upgrades, options).await;
    } else {
        fl_info!("no-upgrades-aur-package");
    }

    fl_info!("scanning-for-pacnew");
    detect().await;
}
