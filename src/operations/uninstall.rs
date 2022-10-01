use std::env;
use std::path::Path;
use tokio::fs;

use crate::builder::pacman::PacmanUninstallBuilder;
use crate::internal::exit_code::AppExitCode;
use crate::{fl_crash, Options};

/// Uninstalls the given packages
#[tracing::instrument(level = "trace")]
pub async fn uninstall(packages: Vec<String>, options: Options) {
    tracing::debug!("Uninstalling: {:?}", &packages);

    PacmanUninstallBuilder::default()
        .recursive(true)
        .no_confirm(options.noconfirm)
        .packages(&packages)
        .uninstall()
        .await
        .unwrap_or_else(|_| {
            fl_crash!(AppExitCode::PacmanError, "failed-remove-pkgs");
        });

    for package in packages {
        if Path::new(&format!(
            "{}/.cache/ame/{}",
            env::var("HOME").unwrap(),
            package
        ))
        .exists()
        {
            tracing::debug!("Old cache directory found, deleting");
            fs::remove_dir_all(Path::new(&format!(
                "{}/.cache/ame/{}",
                env::var("HOME").unwrap(),
                package
            )))
            .await
            .unwrap();
        }
    }
}
