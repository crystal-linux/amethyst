use std::path::Path;
use std::{env, fs};

use crate::error::SilentUnwrap;
use crate::internal::sudo_pacman;
use crate::{log, Options};

pub fn uninstall(packages: Vec<String>, options: Options) {
    let mut pacman_args = vec!["-Rs"];
    pacman_args.append(&mut packages.iter().map(|s| s.as_str()).collect());

    if options.noconfirm {
        pacman_args.push("--noconfirm");
    }
    let verbosity = options.verbosity;
    if verbosity >= 1 {
        log(format!("Uninstalling: {:?}", &packages));
    }

    sudo_pacman(pacman_args).silent_unwrap();

    if verbosity >= 1 {
        log(format!(
            "Uninstalling packages: {:?} exited with code 0",
            &packages
        ));
    }

    for package in packages {
        crate::database::remove(&package, options);
        if Path::new(&format!(
            "{}/.cache/ame/{}",
            env::var("HOME").unwrap(),
            package
        ))
        .exists()
        {
            if verbosity >= 1 {
                log("Old cache directory found, deleting".to_string());
            }
            fs::remove_dir_all(Path::new(&format!(
                "{}/.cache/ame/{}",
                env::var("HOME").unwrap(),
                package
            )))
            .unwrap();
        }
    }
}
