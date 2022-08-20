use std::env;
use std::path::Path;

use crate::{crash, internal::exit_code::AppExitCode, log, Options};

pub fn init(options: Options) {
    // Initialise variables
    let verbosity = options.verbosity;
    let homedir = env::var("HOME").unwrap();

    // If stateful dir doesn't exist, create it
    if !Path::new(&format!("{}/.local/share/ame/", homedir)).exists() {
        if verbosity >= 1 {
            log!("Initialising stateful directory");
        }
        std::fs::create_dir_all(format!("{}/.local/share/ame", homedir)).unwrap_or_else(|e| {
            crash!(
                AppExitCode::FailedCreatingPaths,
                "Couldn't create path: {}/.local/share/ame: {}",
                homedir,
                e,
            );
        });
    }
}
