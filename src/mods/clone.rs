use git2::Repository;
use std::{env, fs, path::Path, process::Command};

pub fn clone(pkg: &str, cachedir: &str) {
    let error = format!("Couldn't install {}", &pkg);
    let path = Path::new(&cachedir);
    let pkgdir=format!("{}/{}", &cachedir, &pkg);
    let pkgpath = Path::new(&pkgdir);
    env::set_current_dir(&pkgdir);
    fs::create_dir(&pkg);
    let results = raur::search(&pkg).expect(&error);
    let url = format!("https://aur.archlinux.org/{}.git", results[0].name);
    println!("Cloning {} ...", pkg);
    println!("{}", &cachedir);
    Repository::clone(&url, &pkgpath).unwrap();
    env::set_current_dir(&pkgpath);
    println!("Installing {} ...", pkg);
    Command::new("makepkg")
        .arg("-si")
        .status()
        .expect(&error);
}
