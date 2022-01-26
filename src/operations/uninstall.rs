use std::path::Path;
use std::{env, fs};

use crate::{log, Options};

pub fn uninstall(mut a: Vec<String>, options: Options) {
    let b = a.clone();
    if options.noconfirm {
        a.push("--noconfirm".to_string());
    }
    let verbosity = options.verbosity;
    if verbosity >= 1 {
        log(format!("Uninstalling: {:?}", &b));
    }

    let r = runas::Command::new("pacman")
        .arg("-Rs")
        .args(&a)
        .status()
        .expect("Something has gone wrong");

    if let Some(x) = r.code() {
        if verbosity >= 1 {
            log(format!(
                "Uninstalling packages: {:?} exited with code {}",
                &b, x
            ));
        }
    }

    for b in a {
        crate::database::remove(&b, options);
        if Path::new(&format!("{}/.cache/ame/{}", env::var("HOME").unwrap(), b)).exists() {
            if verbosity >= 1 {
                log("Old cache directory found, deleting".to_string());
            }
            fs::remove_dir_all(Path::new(&format!(
                "{}/.cache/ame/{}",
                env::var("HOME").unwrap(),
                b
            )))
            .unwrap();
        }
    }
}
