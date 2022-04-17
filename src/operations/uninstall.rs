use std::path::Path;
use std::{env, fs};

use crate::internal::commands::ShellCommand;
use crate::internal::error::SilentUnwrap;
use crate::internal::exit_code::AppExitCode;
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

    ShellCommand::pacman()
        .elevated()
        .args(pacman_args)
        .wait_success()
        .silent_unwrap(AppExitCode::PacmanError);

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
