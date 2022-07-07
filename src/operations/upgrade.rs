use crate::internal::commands::ShellCommand;
use crate::internal::error::SilentUnwrap;
use crate::internal::exit_code::AppExitCode;
use crate::internal::rpc::rpcinfo;
use crate::operations::aur_install::aur_install;
use crate::{info, log, prompt, Options};

/// Upgrades all installed packages
pub async fn upgrade(options: Options) {
    let verbosity = options.verbosity;
    let noconfirm = options.noconfirm;

    let mut pacman_args = vec!["-Syu"];
    if noconfirm {
        pacman_args.push("--noconfirm");
    }

    if verbosity >= 1 {
        log!("Upgrading repo packages");
    }

    let pacman_result = ShellCommand::pacman()
        .elevated()
        .args(pacman_args)
        .wait()
        .await
        .silent_unwrap(AppExitCode::PacmanError);

    if pacman_result.success() {
        info!("Successfully upgraded repo packages");
    } else {
        let continue_upgrading = prompt!(default false,
            "Failed to upgrade repo packages, continue to upgrading AUR packages?",
        );
        if !continue_upgrading {
            info!("Exiting");
            std::process::exit(AppExitCode::PacmanError as i32);
        }
    }

    if verbosity >= 1 {
        log!("Upgrading AUR packages");
    }

    let packages = crate::database::query(options);

    if verbosity >= 1 {
        log!("{:?}", &packages);
    }
    let mut aur_upgrades = vec![];

    for package in packages {
        let remote_package = rpcinfo(&package.name);

        if remote_package.package.unwrap().version != package.version {
            aur_upgrades.push(package.name);
        }
    }

    if !aur_upgrades.is_empty() {
        aur_install(aur_upgrades, options).await;
    } else {
        info!("No upgrades available for installed AUR packages");
    }
}
