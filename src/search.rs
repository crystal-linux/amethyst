use std::{ops::Deref, process::Command};

pub fn a_search(pkg: &str) {
    let results = raur::search(&pkg);
    for res in &results {
        println!("aur/{} {}\n    {}", res[0].name, res[0].version, res[0].description.as_ref().map_or("n/a", String::deref));
    }
}

pub fn r_search(pkg: &str) {
    let errstr = format!("Something happened");
    Command::new("pacman")
        .arg("-Ss")
        .arg(&pkg)
        .status()
        .expect(&errstr);    
}
