use runas::Command;
use git2::Repository;
use std::{path, env};

// fix unused std::result::Result

pub fn upgrade(noconfirm: bool, cachedir: &str){
    let errstr = format!("Something happened");
    if noconfirm == true {
        Command::new("pacman")
            .arg("-Syu")
            .arg("--noconfirm")
            .status()
            .expect(&errstr);
    } else {
        Command::new("pacman")
            .arg("-Syu")
            .status()
            .expect(&errstr);
    }
    for file in std::fs::read_dir(&cachedir).unwrap() {
        let dir = &file.unwrap().path();
        env::set_current_dir(&dir);
        let output = std::process::Command::new("git").arg("pull").output().unwrap(); //figure out how to pull with the git2 crate!
        let update_available = String::from_utf8(output.stdout).unwrap();
        if update_available != "Already up to date." {
            let path_as_str = &dir.display().to_string();
            let pkg: Vec<&str> = path_as_str.split("/").collect();
            println!("{} is up to date", pkg[pkg.len()-1]);
        } else {
            env::set_current_dir(&dir);
            std::process::Command::new("makepkg").arg("-si").status();
        }
    }
}   
