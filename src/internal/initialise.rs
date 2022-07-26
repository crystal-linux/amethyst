use std::env;
use std::path::Path;
use std::process::Command;

use crate::{crash, internal::exit_code::AppExitCode, log, Options};

pub fn init(options: Options) {
    // Initialise variables
    let verbosity = options.verbosity;
    let homedir = env::var("HOME").unwrap();

    // Initialise stateful directory
    if !Path::new(&format!("{}/.local/share/ame", homedir)).exists() {
        std::fs::create_dir_all(format!("{}/.local/share/ame", homedir)).unwrap_or_else(|e| {
            crash!(
                AppExitCode::FailedCreatingPaths,
                "Failed to create stateful directory: {}",
                e
            );
        });
    }

    // If cache path doesn't exist, create it, if it does, delete it and recreate it
    if !Path::new(&format!("{}/.cache/ame/", homedir)).exists() {
        std::fs::create_dir_all(format!("{}/.cache/ame", homedir)).unwrap_or_else(|e| {
            crash!(
                AppExitCode::FailedCreatingPaths,
                "Couldn't create path: {}/.cache/ame: {}",
                homedir,
                e,
            );
        });
    } else {
        rm_rf::remove(format!("{}/.cache/ame", homedir)).unwrap_or_else(|e| {
            crash!(
                AppExitCode::FailedCreatingPaths,
                "Couldn't remove path: {}/.cache/ame: {}",
                homedir,
                e
            )
        });
        std::fs::create_dir_all(format!("{}/.cache/ame", homedir)).unwrap_or_else(|e| {
            crash!(
                AppExitCode::FailedCreatingPaths,
                "Couldn't create path: {}/.cache/ame: {}",
                homedir,
                e
            )
        });
    }
}
