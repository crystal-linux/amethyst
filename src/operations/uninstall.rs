use std::env;
use std::path::Path;
use tokio::fs;

use crate::internal::commands::ShellCommand;
use crate::internal::error::SilentUnwrap;
use crate::internal::exit_code::AppExitCode;
use crate::Options;

/// Uninstalls the given packages
#[tracing::instrument(level = "trace")]
pub async fn uninstall(packages: Vec<String>, options: Options) {
    let mut pacman_args = vec!["-Rs"];
    pacman_args.append(&mut packages.iter().map(|s| s.as_str()).collect());

    if options.noconfirm {
        pacman_args.push("--noconfirm");
    }
    tracing::debug!("Uninstalling: {:?}", &packages);

    ShellCommand::pacman()
        .elevated()
        .args(pacman_args)
        .wait_success()
        .await
        .silent_unwrap(AppExitCode::PacmanError);

    tracing::debug!("Uninstalling packages: {:?} exited with code 0", &packages);

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
