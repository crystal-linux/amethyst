use crate::{info, log, Options};

pub fn install(mut a: Vec<String>, options: Options) {
    info(format!("Installing packages {} from repos", &a.join(", ")));
    let b = a.clone();
    if options.noconfirm {
        a.push("--noconfirm".to_string());
    }
    if options.asdeps {
        a.push("--asdeps".to_string());
    }
    let verbosity = options.verbosity;
    if verbosity >= 1 {
        log(format!("Installing from repos: {:?}", &b));
    }

    let r = runas::Command::new("pacman")
        .arg("-S")
        .arg("--needed")
        .args(&a)
        .status()
        .expect("Something has gone wrong");

    if let Some(x) = r.code() {
        if verbosity >= 1 {
            log(format!(
                "Installing packages: {:?} exited with code {}",
                &b, x
            ));
        }
    }
}
