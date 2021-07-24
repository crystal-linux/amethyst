use git2::Repository;
use std::{fs, path::Path};

pub fn clone(pkg: &str) {
    let cachedir = format!("{}/.cache/ame/{}", std::env::var("HOME").unwrap(), pkg);
    let error = format!("Package {} not found.", &pkg);
    let path = Path::new(&cachedir);
    let results = raur::search(&pkg).expect(&error);
    let url = format!("https://aur.archlinux.org/{}.git", &pkg);

    if path.exists() {
        fs::remove_dir_all(path).unwrap();
    }
              
    for _res in results.first() {
        println!("Cloning {} ...", pkg);
        Repository::clone(&url, &path).unwrap();
    }
}

