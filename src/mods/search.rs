use crate::mods::strs::{err_rec, err_unrec, succ};
use ansi_term::Colour;
use std::{ops::Deref, process::Command};

pub fn a_search(pkg: &str) {
    // search for a package in the AUR
    let results = raur::search(&pkg);

    for r in &results {
        if r.is_empty() {
            err_rec("No matching AUR packages found".to_string());
        }
        for res in r {
            println!(
                "{}{} {}\n    {}",
                Colour::Cyan.bold().paint("aur/"),
                Colour::White.bold().paint(&res.name),
                Colour::Green.bold().paint(&res.version),
                Colour::White.paint(res.description.as_ref().map_or("n/a", String::deref))
            );
        }
    }
}

pub fn r_search(pkg: &str) {
    // search for a package in the repositories
    let result = Command::new("pacman")
        .arg("-Ss")
        .arg(&pkg)
        .status()
        .unwrap();
    match result.code() {
        Some(0) => succ("Repo search successful".to_string()),
        Some(1) => err_rec("No matching repo packages found".to_string()),
        Some(_) => err_unrec("Someting went terribly wrong".to_string()),
        None => err_unrec("Couldn't search pacman repos".to_string()),
    };
}
