use std::process::Command;

use crate::internal::rpc::rpcsearch;
use crate::{log, Options};

pub fn aur_search(a: &str, options: Options) {
    let verbosity = options.verbosity;
    let res = rpcsearch(a.to_string());

    if verbosity >= 1 {
        log(format!(
            "Found {} resuls for \"{}\" in AUR",
            res.resultcount, a
        ));
    }

    for r in &res.results {
        println!(
            "aur/{} {}\n    {}",
            r.name,
            r.version,
            r.description
                .as_ref()
                .unwrap_or(&"No description".to_string())
        )
    }
}

pub fn repo_search(a: &str, options: Options) {
    let verbosity = options.verbosity;
    let rs = Command::new("pacman")
        .arg("-Ss")
        .arg(&a)
        .output()
        .expect("Something has gone wrong");

    let str = String::from_utf8(rs.stdout).unwrap();

    if verbosity >= 1 {
        log(format!(
            "Found {} results for \"{}\" in repos",
            &str.split('\n').count() / 2,
            &a
        ));
    }

    print!("{}", str);
}
