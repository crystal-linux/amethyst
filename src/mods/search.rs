use crate::mods::strs::{err_rec, err_unrec, succ};
use ansi_term::Colour;
use std::{ops::Deref, process::Command};

pub fn a_search(pkg: &str) {
    let results = raur::search(&pkg);

    for r in &results {
        if r.len() == 0 {
            err_rec(format!("No matching AUR packages found"));
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
    let result = Command::new("pacman")
        .arg("-Ss")
        .arg(&pkg)
        .status()
        .unwrap();
    match result.code() {
        Some(0) => succ(format!("Repo search successful")),
        Some(1) => err_rec(format!("No matching repo packages found")),
        Some(_) => err_unrec(format!("Someting went terribly wrong")),
        None => err_unrec(format!("Couldn't search pacman repos")),
    };
}
