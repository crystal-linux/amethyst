use std::{ops::Deref, process::Command};
use crate::mods::strs::{err_unrec, inf};

pub fn a_search(pkg: &str) {
    let results = raur::search(&pkg);

    for res in &results {
        if res.len() <= 1 {
            err_unrec(format!("No matching packages found"));
        }
        println!("aur/{} {}\n    {}",
        res[0].name,
        res[0].version,
        res[0].description.as_ref().map_or("n/a", String::deref));
    }
}

pub fn r_search(pkg: &str) {
    let result = Command::new("pacman")
                         .arg("-Ss")
                         .arg(&pkg)
                         .status();
    match result {
    Ok(_) => {
        inf(format!("Repo search successful"))
    }
    Err(_) => {
        err_unrec(format!("Couldn't search pacman repos"))
    }};

}
