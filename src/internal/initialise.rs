use std::env;
use std::path::Path;

use crate::{crash, internal::exit_code::AppExitCode, log, Options};

pub fn init(options: Options) {
    // Initialise variables
    let verbosity = options.verbosity;
    let homedir = env::var("HOME").unwrap();

    // Initialise stateful directory
    if !Path::new(&format!("{}/.local/share/ame", homedir)).exists() {
        if verbosity >= 1 {
            log!("Initialising stateful directory");
        }
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
        if verbosity >= 1 {
            log!("Initialising cache directory");
        }
        std::fs::create_dir_all(format!("{}/.cache/ame", homedir)).unwrap_or_else(|e| {
            crash!(
                AppExitCode::FailedCreatingPaths,
                "Couldn't create path: {}/.cache/ame: {}",
                homedir,
                e,
            );
        });
    } else {
        if verbosity >= 1 {
            log!("Deleting cache directory");
        }
        rm_rf::remove(format!("{}/.cache/ame", homedir)).unwrap_or_else(|e| {
            crash!(
                AppExitCode::FailedCreatingPaths,
                "Couldn't remove path: {}/.cache/ame: {}",
                homedir,
                e
            )
        });
        if verbosity >= 1 {
            log!("Creating cache directory");
        }
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
