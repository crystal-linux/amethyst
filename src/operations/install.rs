use crate::Options;

pub fn install(mut a: Vec<String>, options: Options) {
    let b = a.clone();
    if options.noconfirm {
        a.push("--noconfirm".to_string());
    }
    if options.asdeps {
        a.push("--asdeps".to_string());
    }
    let verbosity = options.verbosity;
    match verbosity {
        0 => {}
        1 => {
            eprintln!("Installing from repos:");
            eprintln!("{:?}", &b);
        }
        _ => {
            eprintln!("Installing from repos:");
            for b in &a {
                eprintln!("{:?}", b);
            }
        }
    }

    let r = runas::Command::new("pacman")
        .arg("-S")
        .arg("--needed")
        .args(&a)
        .status()
        .expect("Something has gone wrong.");

    if let Some(x) = r.code() {
        if verbosity >= 1 {
            eprintln!("Installing packages: {:?} exited with code {}", &b, x)
        }
    }
}
