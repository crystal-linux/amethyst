use crate::crash;
use crate::internal::commands::ShellCommand;

use crate::internal::error::SilentUnwrap;
use crate::internal::exit_code::AppExitCode;

use crate::internal::utils::get_cache_dir;
use crate::prompt;
use crate::Options;

/// Removes orphaned packages and cache
#[tracing::instrument(level = "trace")]
pub async fn clean(options: Options) {
    let noconfirm = options.noconfirm;

    // Check for orphaned packages
    let orphaned_packages = ShellCommand::pacman()
        .arg("-Qdtq")
        .wait_with_output()
        .await
        .silent_unwrap(AppExitCode::PacmanError);

    if orphaned_packages.stdout.as_str().is_empty() {
        // If no orphaned packages found, do nothing
        tracing::info!("No orphaned packages found");
    } else {
        // Prompt users whether to remove orphaned packages
        tracing::info!(
            "Removing orphans would uninstall the following packages: \n{}",
            &orphaned_packages.stdout
        );
        let cont = prompt!(default no, "Continue?");
        if !cont {
            // If user doesn't want to continue, break
            tracing::info!("Exiting");
            std::process::exit(AppExitCode::PacmanError as i32);
        }

        // Build pacman args
        let mut pacman_args = vec!["-Rns"];
        if noconfirm {
            pacman_args.push("--noconfirm");
        }

        // Collect orphaned packages into a vector
        let orphaned_packages_vec = orphaned_packages.stdout.split('\n').collect::<Vec<&str>>();
        for package in &orphaned_packages_vec {
            if !package.is_empty() {
                pacman_args.push(package);
            }
        }

        tracing::debug!("Removing orphans: {:?}", orphaned_packages_vec);

        // Remove orphaned packages
        let pacman_result = ShellCommand::pacman()
            .elevated()
            .args(pacman_args)
            .wait()
            .await
            .silent_unwrap(AppExitCode::PacmanError);

        if pacman_result.success() {
            // If pacman succeeded, notify user
            tracing::info!("Successfully removed orphans");
        } else {
            // If pacman failed, crash
            crash!(AppExitCode::PacmanError, "Failed to remove orphans",);
        }
    }

    // Prompt the user whether to clear the Amethyst cache
    let clear_ame_cache = prompt!(default no, "Clear Amethyst's internal PKGBUILD cache?");
    if clear_ame_cache {
        let cache_dir = get_cache_dir();
        ShellCommand::rm()
            .arg(cache_dir)
            .arg("-r")
            .arg("-f")
            .elevated()
            .wait_success()
            .await
            .unwrap();
    }

    // Prompt the user whether to clear cache or not
    let clear_pacman_cache = if noconfirm {
        true
    } else {
        prompt!(default no, "Also clear pacman's package cache?")
    };

    if clear_pacman_cache {
        // Build pacman args
        let mut pacman_args = vec!["-Sc"];
        if noconfirm {
            pacman_args.push("--noconfirm");
        }

        // Build paccache args
        let mut paccache_args = vec!["-r"];
        if noconfirm {
            paccache_args.push("--noconfirm");
        }

        tracing::debug!("Clearing using `paccache -r`");

        // Clear pacman's cache (keeping latest 3 versions of installed packages)
        ShellCommand::sudo()
            .arg("paccache")
            .args(paccache_args)
            .elevated()
            .spawn(true)
            .unwrap_or_else(|e| {
                crash!(
                    AppExitCode::PacmanError,
                    "Couldn't clear cache using `paccache -r`, {}",
                    e
                )
            })
            .wait()
            .await
            .unwrap();

        tracing::debug!("Clearing using `pacman -Sc`");

        // Clear pacman's cache (keeping only installed packages)
        let pacman_result = ShellCommand::pacman()
            .elevated()
            .args(pacman_args)
            .wait()
            .await
            .silent_unwrap(AppExitCode::PacmanError);

        if pacman_result.success() {
            // If pacman succeeded, notify user
            tracing::info!("Successfully cleared package cache");
        } else {
            // If pacman failed, crash
            crash!(AppExitCode::PacmanError, "Failed to clear package cache",);
        }
    }
}
