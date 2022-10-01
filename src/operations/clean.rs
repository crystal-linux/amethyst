use crate::builder::paccache::PaccacheBuilder;
use crate::builder::pacman::PacmanQueryBuilder;
use crate::builder::pacman::PacmanUninstallBuilder;
use crate::builder::rm::RmBuilder;

use crate::fl;
use crate::fl_crash;
use crate::fl_info;
use crate::fl_prompt;
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
        fl_info!("no-orphans");
    } else {
        // Prompt users whether to remove orphaned packages
        fl_info!(
            "removing-orphans-would",
            packages = orphaned_packages.stdout.trim_end()
        );
        let cont = noconfirm || prompt!(default no, "Continue?");
        if !cont {
            // If user doesn't want to continue, break
            fl_info!("exiting");
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

        if result.is_err() {
            fl_crash!(AppExitCode::PacmanError, "failed-remove-orphans");
        } else {
            fl_info!("success-remove-orphans");
        }
    }

    // Prompt the user whether to clear the Amethyst cache
    let clear_ame_cache = noconfirm || fl_prompt!(default no, "clear-pkgbuild-cache");

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
    let clear_pacman_cache = noconfirm || fl_prompt!(default no, "clear-pacman-cache");

    if clear_pacman_cache {
        // Clear pacman's cache
        // keeps 3 versions of the package in the cache
        // keeps installed packages in the cache
        let result = PaccacheBuilder::default()
            .set_keep(3)
            .keep_ins_pkgs(true)
            .quiet_output(quiet)
            .remove()
            .await;

        if let Err(e) = result {
            fl_crash!(
                AppExitCode::PacmanError,
                "failed-clear-cache",
                error = e.to_string()
            );
        } else {
            fl_info!("success-clear-cache");
        }
    }
}
