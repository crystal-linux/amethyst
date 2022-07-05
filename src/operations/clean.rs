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
        info!("No orphaned packages found");
    } else {
        info!(
            "Removing orphans would uninstall the following packages: \n{}",
            &orphaned_packages.stdout
        );
        let cont = prompt!(default false, "Continue?");
        if !cont {
            info!("Exiting");
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
            log!("Removing orphans: {:?}", orphaned_packages_vec);
        }

        let pacman_result = ShellCommand::pacman()
            .elevated()
            .args(pacman_args)
            .wait()
            .silent_unwrap(AppExitCode::PacmanError);

        if pacman_result.success() {
            info!("Successfully removed orphans");
        } else {
            crash!(AppExitCode::PacmanError, "Failed to remove orphans",);
        }
    }

    let clear_cache = if !noconfirm {
        prompt!(default false, "Also clear pacman's package cache?")
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
            log!("Clearing using `paccache -r`");
        }

        Command::new("sudo")
            .arg("paccache")
            .args(paccache_args)
            .spawn()
            .unwrap_or_else(|e| {
                crash!(
                    AppExitCode::PacmanError,
                    "Couldn't clear cache using `paccache -r`, {}",
                    e,
                )
            })
            .wait()
            .unwrap();

        if verbosity >= 1 {
            log!("Clearing using `pacman -Sc`");
        }

        let pacman_result = ShellCommand::pacman()
            .elevated()
            .args(pacman_args)
            .wait()
            .silent_unwrap(AppExitCode::PacmanError);

        if pacman_result.success() {
            info!("Successfully cleared package cache");
        } else {
            crash!(AppExitCode::PacmanError, "Failed to clear package cache",);
        }
    }
}
