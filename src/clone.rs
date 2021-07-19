use git2::Repository;
use std::{fs, path::Path, process::exit};
use serde_json::Value;
use std::process::Command;
use std::io::{Error, ErrorKind};

pub fn clone(pkg: &str) {
    let url = format!("https://aur.archlinux.org/{}.git", pkg);
    let aurl = format!("https://aur.archlinux.org/packages/{}", pkg);
    let homedir = std::env::var("HOME").unwrap();
    let cachedir = format!("{}/.cache/ame/{}", homedir, pkg);
    let path = Path::new(&cachedir);
    let errcode = Command::new("pacman").arg("-Ss").arg(&pkg).status().unwrap();

    if errcode.success() {
        println!("found {} in repos!", &pkg);
        Command::new("sudo").arg("pacman").arg("-S").arg(&pkg).spawn();
        
    } else {
        println!("error");

        if path.exists() {
            fs::remove_dir_all(path).unwrap();
        }
        
        let aresp = ureq::get(&aurl).call().unwrap_or_else(|error| {
            println!("{}", error);
            exit(1);
        });
        
        if aresp.status() == 200 {
            println!("Cloning {} ...", pkg);
            Repository::clone(&url, &path).unwrap();
        } else {
            println!("Error, repository {} not found", pkg);
        }
    }
}
