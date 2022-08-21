use std::process::{Command, Stdio};

use crate::internal::{clean, rpc, structs};
use crate::{log, Options};

pub fn sort(input: &[String], options: Options) -> structs::Sorted {
    // Initialise variables
    let mut repo: Vec<String> = vec![];
    let mut aur: Vec<String> = vec![];
    let mut nf: Vec<String> = vec![];
    let verbosity = options.verbosity;

    // Sanitise all packages passed in
    let a = clean(input, options);

    if verbosity >= 1 {
        log!("Sorting: {:?}", a.join(" "));
    }

    for b in a {
        // Check if package is in the repos
        let rs = Command::new("pacman")
            .arg("-Ss")
            .arg(format!("^{}$", &b))
            .stdout(Stdio::null())
            .status()
            .expect("Something has gone wrong");

        if rs.code() == Some(0) {
            // If it is, add it to the repo vector
            if verbosity >= 1 {
                log!("{} found in repos", b);
            }
            repo.push(b.to_string());
        } else if rpc::rpcinfo(&b).found {
            // Otherwise, check if it is in the AUR, if it is, add it to the AUR vector
            if verbosity >= 1 {
                log!("{} found in AUR", b);
            }
            aur.push(b.to_string());
        } else {
            // Otherwise, add it to the not found vector
            if verbosity >= 1 {
                log!("{} not found", b);
            }
            nf.push(b.to_string());
        }
    }

    structs::Sorted::new(repo, aur, nf)
}
