use crate::internal::{clean, rpc, structs};
use crate::Options;
use std::process::{Command, Stdio};

pub fn sort(input: &[String], options: Options) -> structs::Sorted {
    let mut repo: Vec<String> = vec![];
    let mut aur: Vec<String> = vec![];
    let mut nf: Vec<String> = vec![];
    let verbosity = options.verbosity;

    let a = clean(input, options);

    match verbosity {
        0 => {}
        1 => {
            eprintln!("Sorting:");
            eprintln!("{:?}", a);
        }
        _ => {
            eprintln!("Sorting:");
            for b in &a {
                eprintln!("{}", b);
            }
        }
    }

    for b in a {
        let rs = Command::new("pacman")
            .arg("-Ss")
            .arg(format!("^{}$", &b))
            .stdout(Stdio::null())
            .status()
            .expect("Something has gone wrong.");

        if rpc::rpcinfo(b.to_string()).found {
            if verbosity >= 1 {
                eprintln!("{} found in AUR.", b);
            }
            aur.push(b.to_string());
        } else if let Some(0) = rs.code() {
            if verbosity >= 1 {
                eprintln!("{} found in repos.", b)
            }
            repo.push(b.to_string());
        } else {
            if verbosity >= 1 {
                eprintln!("{} not found.", b);
            }
            nf.push(b.to_string());
        }
    }

    structs::Sorted::new(repo, aur, nf)
}
