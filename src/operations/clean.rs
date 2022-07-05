use std::process::Command;

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
        .arg("-Qdtq")
        .wait_with_output()
        .silent_unwrap(AppExitCode::PacmanError);

    if orphaned_packages.stdout.as_str() == "" {
        info("No orphaned packages found".to_string());
    } else {
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

        let orphaned_packages_vec = orphaned_packages.stdout.split('\n').collect::<Vec<&str>>();
        for package in &orphaned_packages_vec {
            if !package.is_empty() {
                pacman_args.push(package);
            }
        }

        if verbosity >= 1 {
            log(format!("Removing orphans: {:?}", orphaned_packages_vec));
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
    }

    let clear_cache = if !noconfirm {
        prompt("Also clear pacman's package cache?".to_string(), false)
    } else {
        true
    };
    if clear_cache {
        let mut pacman_args = vec!["-Sc"];
        if noconfirm {
            pacman_args.push("--noconfirm");
        }

        let mut paccache_args = vec!["-r"];
        if noconfirm {
            paccache_args.push("--noconfirm");
        }

        if verbosity >= 1 {
            log("Clearing using `paccache -r`".to_string());
        }

        Command::new("sudo")
            .arg("paccache")
            .args(paccache_args)
            .spawn()
            .unwrap_or_else(|e| {
                crash(
                    format!("Couldn't clear cache using `paccache -r`, {}", e),
                    AppExitCode::PacmanError,
                )
            })
            .wait()
            .unwrap();

        if verbosity >= 1 {
            log("Clearing using `pacman -Sc`".to_string());
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
