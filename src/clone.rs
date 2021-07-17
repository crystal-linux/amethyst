use git2::Repository;
use std::{fs, path::Path, process::exit};
use serde_json::Value;

pub fn clone(pkg: &str) {
    let url = format!("https://aur.archlinux.org/{}.git", pkg);
    let aurl = format!("https://aur.archlinux.org/packages/{}", pkg);
    let homedir = std::env::var("HOME").unwrap();
    let cachedir = format!("{}/.cache/ame/{}", homedir, pkg);
    let path = Path::new(&cachedir);

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

    println!("{}", deps);

}
