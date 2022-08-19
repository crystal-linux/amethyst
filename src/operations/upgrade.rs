use crate::internal::commands::ShellCommand;
use crate::internal::detect;
use crate::internal::error::SilentUnwrap;
use crate::internal::exit_code::AppExitCode;
use crate::internal::rpc::rpcinfo;
use crate::operations::aur_install::aur_install;
use crate::{info, log, prompt, Options};

#[derive(Debug)]
struct QueriedPackage {
    pub name: String,
    pub version: String,
}

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
        log!("Checking AUR upgrades...");
    }

    // List non-native packages using `pacman -Qm` and collect to a Vec<String>
    let non_native = ShellCommand::pacman()
        .arg("-Qm")
        .args(&["--color", "never"])
        .wait_with_output()
        .silent_unwrap(AppExitCode::PacmanError);

    // Collect by lines to a Vec<String>
    let mut non_native = non_native.stdout.split('\n').collect::<Vec<&str>>();

    // Remove last element, which is an empty line
    non_native.pop();

    // Parse non-native packages into a Vec<QueriedPackage>
    let mut parsed_non_native: Vec<QueriedPackage> = vec![];
    for pkg in non_native {
        // Split by space
        let split = pkg.split(' ').collect::<Vec<&str>>();
        if verbosity >= 1 {
            log!("{:?}", split);
        }
        // Create QueriedPackage and push it to parsed_non_native
        let name = split[0].to_string();
        let version = split[1].to_string();
        parsed_non_native.push(QueriedPackage { name, version });
    }

    if verbosity >= 1 {
        log!("{:?}", &parsed_non_native);
    }

    // Check if AUR package versions are the same as installed
    let mut aur_upgrades = vec![];
    for pkg in parsed_non_native {
        // Query AUR
        let rpc_result = rpcinfo((&*pkg.name).to_string());

        if !rpc_result.found {
            // If package not found, skip
            continue;
        }

        // If versions differ, push to a vector
        if rpc_result.package.unwrap().version != pkg.version {
            aur_upgrades.push(pkg.name);
        }
    }

    // If vector isn't empty, prompt to install AUR packages from vector, effectively upgrading
    if !aur_upgrades.is_empty() {
        let cont = prompt!(default false,
            "Found AUR packages {} have new versions available, upgrade?",
            aur_upgrades.join(", "),
        );
        if cont {
            aur_install(aur_upgrades, options);
        };
    } else {
        info!("No upgrades available for installed AUR packages");
    }

    // Check for .pacnew files
    detect();
}
