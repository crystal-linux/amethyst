use crate::internal::sudo_pacman;
use crate::{crash, info, log, Options};

pub fn install(a: Vec<String>, options: Options) {
    info(format!("Installing packages {} from repos", &a.join(", ")));
    let mut opers = vec!["-S", "--needed"];
    if options.noconfirm {
        opers.push("--noconfirm");
    }
    if options.asdeps {
        opers.push("--asdeps");
    }
    let verbosity = options.verbosity;
    if !a.is_empty() {
        if verbosity >= 1 {
            log(format!("Installing from repos: {:?}", &a));
        }

        if let Err(_e) = sudo_pacman(&opers) {
            crash(
                format!(
                    "An error occured while installing packages: {}, aborting",
                    a.join(", ")
                ),
                7,
            );
        }

        if verbosity >= 1 {
            log(format!("Installing packages: {:?} was successful", &a));
        }
    }
}
