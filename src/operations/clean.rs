use crate::builder::paccache::PaccacheBuilder;
use crate::builder::pacman::PacmanQueryBuilder;
use crate::builder::pacman::PacmanUninstallBuilder;
use crate::builder::rm::RmBuilder;
use crate::crash;

use crate::internal::config::Config;
use crate::internal::exit_code::AppExitCode;

use crate::internal::utils::get_cache_dir;
use crate::prompt;
use crate::Options;

/// Removes orphaned packages and cache
#[tracing::instrument(level = "trace")]
pub async fn clean(options: Options) {
    let noconfirm = options.noconfirm;
    let quiet = options.quiet;

    // Check for orphaned packages
    let orphaned_packages = PacmanQueryBuilder::orphaned()
        .query_as_string_output()
        .await
        .unwrap();

    if orphaned_packages.stdout.as_str().is_empty() {
        // If no orphaned packages found, do nothing
        tracing::info!("No orphaned packages found");
    } else {
        // Prompt users whether to remove orphaned packages
        tracing::info!(
            "Removing orphans would uninstall the following packages: \n{}",
            &orphaned_packages.stdout.trim_end()
        );
        let cont = prompt!(default no, "Continue?");
        if !cont {
            // If user doesn't want to continue, break
            tracing::info!("Exiting");
            std::process::exit(AppExitCode::PacmanError as i32);
        }

        // Collect orphaned packages into a vector
        let orphaned_packages_vec = orphaned_packages
            .stdout
            .trim_end()
            .split('\n')
            .collect::<Vec<&str>>();

        tracing::debug!("Removing orphans: {:?}", orphaned_packages_vec);

        // Remove orphaned packages
        let result = PacmanUninstallBuilder::default()
            .no_save(true)
            .recursive(true)
            .no_confirm(noconfirm)
            .packages(orphaned_packages_vec)
            .uninstall()
            .await;

        if let Err(_) = result {
            crash!(AppExitCode::PacmanError, "Failed to remove orphans");
        } else {
            tracing::info!("Successfully removed orphans");
        }
    }

    // Prompt the user whether to clear the Amethyst cache
    let clear_ame_cache = prompt!(default no, "Clear Amethyst's internal PKGBUILD cache?");
    if clear_ame_cache {
        let cache_dir = get_cache_dir();
        RmBuilder::default()
            .recursive(true)
            .force(true)
            .directory(cache_dir)
            .build()
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
        let conf = Config::read();

        // Clear pacman's cache
        // keeps 0 versions of the package in the cache by default
        // keeps installed packages in the cache by default
        let result = PaccacheBuilder::default()
            .keep(conf.base.paccache_keep)
            .keep_ins(conf.base.paccache_keep_ins)
            .quiet(quiet)
            .remove()
            .await;

        if let Err(e) = result {
            crash!(
                AppExitCode::PacmanError,
                "Failed to clear package cache, {}",
                e
            )
        } else {
            tracing::info!("Successfully cleared package cache");
        }
    }
}
