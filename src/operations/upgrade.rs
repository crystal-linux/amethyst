use crate::internal::commands::ShellCommand;
use crate::internal::error::SilentUnwrap;
use crate::internal::exit_code::AppExitCode;
use crate::internal::rpc::rpcinfo;
use crate::operations::aur_install::aur_install;
use crate::{info, log, prompt, warn, Options};

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

    let pacman_output = ShellCommand::pacman()
        .arg("-Qm")
        .args(&["--color", "never"])
        .wait_with_output()
        .await
        .silent_unwrap(AppExitCode::PacmanError);
    let non_native_pkgs = pacman_output
        .stdout
        .split('\n')
        .filter(|p| !p.is_empty())
        .filter_map(|p| p.split_once(' '))
        .collect::<Vec<(&str, &str)>>();

    if verbosity >= 1 {
        log!("{:?}", non_native_pkgs);
    }
    let mut aur_upgrades = vec![];

    for (pkg_name, pkg_version) in non_native_pkgs {
        if verbosity >= 1 {
            log!(
                "remote package: name = {}, version = {}",
                pkg_name,
                pkg_version
            );
        }
        let remote_package = rpcinfo(pkg_name).await.silent_unwrap(AppExitCode::RpcError);

        if let Some(remote_package) = remote_package {
            if remote_package.metadata.version != pkg_version {
                aur_upgrades.push(pkg_name.to_string());
            }
        } else {
            warn!("Could not find the remote package for {}", pkg_name);
        }
    }

    if !aur_upgrades.is_empty() {
        aur_install(aur_upgrades, options).await;
    } else {
        info!("No upgrades available for installed AUR packages");
    }
}
