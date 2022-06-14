use crate::internal::exit_code::AppExitCode;
use std::env;
use std::path::Path;
use std::process::Command;

use crate::internal::strings::{crash, log};
use crate::Options;

pub fn init(options: Options) {
    let verbosity = options.verbosity;
    let homedir = env::var("HOME").unwrap();

    if !Path::new(&format!("{}/.local/share/ame", homedir)).exists() {
        let r = std::fs::create_dir_all(format!("{}/.local/share/ame", homedir));
        match r {
            Ok(_) => {
                if verbosity >= 1 {
                    log(format!("Created path: {}/.local/share/ame", homedir));
                }
            }
            Err(e) => {
                crash(
                    format!("Couldn't create path: {}/.local/share/ame: {}", homedir, e),
                    AppExitCode::FailedCreatingPaths,
                );
            }
        }
    }

    if !Path::new(&format!("{}/.local/share/ame/db.sqlite", homedir)).exists() {
        crate::database::init(options);
    }

    if !Path::new(&format!("{}/.cache/ame/", homedir)).exists() {
        let r = std::fs::create_dir_all(format!("{}/.cache/ame", homedir));
        match r {
            Ok(_) => {
                if verbosity >= 1 {
                    log(format!("Created path: {}/.cache/ame", homedir));
                }
            }
            Err(e) => {
                crash(
                    format!("Couldn't create path: {}/.cache/ame: {}", homedir, e),
                    AppExitCode::FailedCreatingPaths,
                );
            }
        }
    } else {
        let r = std::fs::remove_dir_all(format!("{}/.cache/ame", homedir));
        match r {
            Ok(_) => {
                if verbosity >= 1 {
                    log(format!("Removing cache: {}/.cache/ame", homedir));
                }
            }
            Err(e) => {
                crash(
                    format!("Couldn't remove path: {}/.cache/ame: {}", homedir, e),
                    AppExitCode::FailedCreatingPaths,
                );
            }
        }
        let r2 = std::fs::create_dir_all(format!("{}/.cache/ame", homedir));
        match r2 {
            Ok(_) => {
                if verbosity >= 1 {
                    log(format!("Created path: {}/.cache/ame", homedir));
                }
            }
            Err(e2) => {
                crash(
                    format!("Couldn't create path: {}/.cache/ame: {}", homedir, e2),
                    AppExitCode::FailedCreatingPaths,
                );
            }
        }
    }

    let r = Command::new("chmod")
        .arg("-R")
        .arg("770")
        .arg(format!("{}/.cache/ame", homedir))
        .status();
    match r {
        Ok(_) => {
            if verbosity >= 1 {
                log(format!(
                    "Set correct permissions for path: {}/.cache/ame",
                    homedir
                ));
            }
        }
        Err(e) => {
            crash(
                format!(
                    "Couldn't set permissions for path: {}/.cache/ame: {}",
                    homedir, e
                ),
                4,
            );
        }
    };
    let r = Command::new("chmod")
        .arg("-R")
        .arg("770")
        .arg(format!("{}/.local/share/ame", homedir))
        .status();
    match r {
        Ok(_) => {
            if verbosity >= 1 {
                log(format!(
                    "Set correct permissions for path: {}/.local/share/ame",
                    homedir
                ));
            }
        }
        Err(e) => {
            crash(
                format!(
                    "Couldn't set permissions for path: {}/.local/share/ame: {}",
                    homedir, e
                ),
                4,
            );
        }
    };
}
