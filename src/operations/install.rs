use crate::{crash, info, log, Options};

pub fn install(a: Vec<String>, options: Options) {
    info(format!("Installing packages {} from repos", &a.join(", ")));
    let mut opers = vec![];
    if options.noconfirm {
        opers.push("--noconfirm".to_string());
    }
    if options.asdeps {
        opers.push("--asdeps".to_string());
    }
    let verbosity = options.verbosity;
    if verbosity >= 1 {
        log(format!("Installing from repos: {:?}", &a));
    }

    let r = runas::Command::new("pacman")
        .arg("-S")
        .arg("--needed")
        .args(&a)
        .args(&opers)
        .status()
        .expect("Something has gone wrong");

    if r.code() != Some(0) {
        crash(
            format!(
                "An error occured while installing packages: {}, aborting",
                a.join(", ")
            ),
            1,
        );
    }

    if verbosity >= 1 {
        log(format!("Installing packages: {:?} was successful", &a));
    }
}
