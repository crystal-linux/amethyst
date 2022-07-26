use crate::internal::commands::ShellCommand;
use crate::internal::error::SilentUnwrap;
use crate::internal::exit_code::AppExitCode;
use crate::internal::rpc::rpcinfo;
use crate::operations::aur_install::aur_install;
use crate::{info, log, prompt, Options};

pub fn upgrade(options: Options) {
    // Initialise variables
    let verbosity = options.verbosity;
    let noconfirm = options.noconfirm;

    // Build pacman args
    let mut pacman_args = vec!["-Syu"];
    if noconfirm {
        pacman_args.push("--noconfirm");
    }

    if verbosity >= 1 {
        log!("Upgrading repo packages");
    }

    // Upgrade repo packages
    let pacman_result = ShellCommand::pacman()
        .elevated()
        .args(pacman_args)
        .wait()
        .silent_unwrap(AppExitCode::PacmanError);

    if pacman_result.success() {
        // If pacman was successful, notify user
        info!("Successfully upgraded repo packages");
    } else {
        // Otherwise, prompt user whether to continue
        let cont = prompt!(default false,
            "Failed to upgrade repo packages, continue to upgrading AUR packages?",
        );
        if !cont {
            // If user doesn't want to continue, break
            info!("Exiting");
            std::process::exit(AppExitCode::PacmanError as i32);
        }
    }

    if verbosity >= 1 {
        log!("Upgrading AUR packages");
    }

    // Query database for AUR packages
    let res = crate::database::query(options);

    if verbosity >= 1 {
        log!("{:?}", &res);
    }

    // Check if AUR package versions are the same as installed
    let mut aur_upgrades = vec![];
    for r in res {
        // Query AUR
        let re = r.clone();
        let ver = rpcinfo(r.name);

        // If versions differ, push to a vector
        if ver.package.unwrap().version != r.version {
            aur_upgrades.push(re.name);
        }
    }

    // If vector isn't empty, install AUR packages from vector, effectively upgrading
    if !aur_upgrades.is_empty() {
        aur_install(aur_upgrades, options);
    } else {
        info!("No upgrades available for installed AUR packages");
    }
}
