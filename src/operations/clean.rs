use crate::crash;
use crate::info;
use crate::internal::commands::ShellCommand;
use crate::internal::error::SilentUnwrap;
use crate::internal::exit_code::AppExitCode;
use crate::log;
use crate::prompt;
use crate::Options;

pub fn clean(options: Options) {
    let verbosity = options.verbosity;
    let noconfirm = options.noconfirm;

    let orphaned_packages = ShellCommand::pacman()
        .arg("-Qdt")
        .wait_with_output()
        .silent_unwrap(AppExitCode::PacmanError);

    info(format!(
        "Removing orphans would uninstall the following packages: \n{}",
        &orphaned_packages.stdout
    ));
    let cont = prompt("Continue?".to_string(), false);
    if !cont {
        info("Exiting".to_string());
        std::process::exit(AppExitCode::PacmanError as i32);
    }

    let mut pacman_args = vec!["-Rns"];
    if noconfirm {
        pacman_args.push("--noconfirm");
    }

    if verbosity >= 1 {
        log("Removing orphans".to_string());
    }

    let pacman_result = ShellCommand::pacman()
        .elevated()
        .args(pacman_args)
        .wait()
        .silent_unwrap(AppExitCode::PacmanError);

    if pacman_result.success() {
        info("Successfully removed orphans".to_string());
    } else {
        crash(
            "Failed to remove orphans".to_string(),
            AppExitCode::PacmanError,
        );
    }

    let clear_cache = prompt("Also clear pacman's package cache?".to_string(), false);
    if clear_cache {
        let mut pacman_args = vec!["-Scc"];
        if noconfirm {
            pacman_args.push("--noconfirm");
        }

        if verbosity >= 1 {
            log("Clearing package cache".to_string());
        }

        let pacman_result = ShellCommand::pacman()
            .elevated()
            .args(pacman_args)
            .wait()
            .silent_unwrap(AppExitCode::PacmanError);

        if pacman_result.success() {
            info("Successfully cleared package cache".to_string());
        } else {
            crash(
                "Failed to clear package cache".to_string(),
                AppExitCode::PacmanError,
            );
        }
    }
}
