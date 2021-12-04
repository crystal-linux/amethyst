use crate::mods::strs::{err_rec, err_unrec, succ};
use crate::mods::rpc::*;
use ansi_term::Colour;
use std::process::Command;

pub fn a_search(pkg: &str) {
    // search for a package in the AUR
    let results = rpcsearch(pkg).results;

    for r in &results {
        if results.is_empty() {
            err_rec("No matching AUR packages found".to_string());
        }
        println!(
            "{}{} {}\n    {}",
            Colour::Cyan.bold().paint("aur/"),
            Colour::White.bold().paint(&r.name),
            Colour::Green.bold().paint(&r.version),
            Colour::White.paint(r.description.as_ref().unwrap_or(&"No description available".to_string()))
        );
    }
    if !results.is_empty() {
        succ("AUR search successful".to_string());
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
