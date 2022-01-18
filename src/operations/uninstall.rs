use crate::Options;

pub fn uninstall(mut a: Vec<String>, options: Options) {
    let b = a.clone();
    if options.noconfirm {
        a.push("--noconfirm".to_string());
    }
    let verbosity = options.verbosity;
    match verbosity {
        0 => {}
        1 => {
            eprintln!("Uninstalling:");
            eprintln!("{:?}", &b);
        }
        _ => {
            eprintln!("Uninstalling:");
            for b in &a {
                eprintln!("{}", b);
            }
        }
    }

    let r = runas::Command::new("pacman")
        .arg("-Rs")
        .args(&a)
        .status()
        .expect("Something has gone wrong.");

    if let Some(x) = r.code() {
        if verbosity >= 1 {
            eprintln!("Uninstalling packages: {:?} exited with code {}.", &b, x)
        }
    }
}
