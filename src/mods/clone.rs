use git2::Repository;
use std::{fs, path::Path, process::Command};

pub fn clone(pkg: &str) {
    let cachedir = format!("{}/.cache/ame/{}", std::env::var("HOME").unwrap(), pkg);
    let error = format!("Couldn't install {}", &pkg);
    let path = Path::new(&cachedir);
    let results = raur::search(&pkg).expect(&error);
    let url = format!("https://aur.archlinux.org/{}.git", results[0].name);
    if path.exists() {
        fs::remove_dir_all(path).unwrap();
    }
    println!("Cloning {} ...", pkg);
    Repository::clone(&url, &path).unwrap();
    println!("Installing {} ...", pkg);
    Command::new("makepkg")
                .current_dir(&cachedir)
                .arg("-si")
                .status()
                .expect(&error);
}
