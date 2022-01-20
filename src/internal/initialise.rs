use crate::internal::strings::{crash, log};
use crate::Options;
use std::env;
use std::path::Path;

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
                    1,
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
                    1,
                );
            }
        }
    }
}
