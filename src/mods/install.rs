use crate::mods::strs::{err_unrec, succ};
use runas::Command;
use std::fs::File;

pub fn install(noconfirm: bool, as_dep: bool, pkg: &str) {
    let pkgs: Vec<&str> = pkg.split(" ").collect();
    if as_dep == false {
        if noconfirm == true {
            let result = Command::new("pacman")
                .arg("-S")
                .arg("--noconfirm")
                .arg("--needed")
                .args(&pkgs)
                .status()
                .expect("Couldn't call pacman");
            match result.code() {
                Some(0) => succ(format!("Succesfully installed packages: {}", pkg)),
                Some(_) => err_unrec(format!("Couldn't install packages: {}", pkg)),
                None => err_unrec(format!("Couldn't install packages: {}", pkg)),
            };
        } else {
            let result = Command::new("pacman")
                .arg("-S")
                .arg("--needed")
                .args(&pkgs)
                .status()
                .expect("Couldn't call pacman");
            match result.code() {
                Some(0) => succ(format!("Succesfully installed packages: {}", pkg)),
                Some(_) => err_unrec(format!("Couldn't install packages: {}", pkg)),
                None => err_unrec(format!("Couldn't install packages: {}", pkg)),
            };
        }
    } else {
        let result = Command::new("pacman")
            .arg("-S")
            .arg("--noconfirm")
            .arg("--needed")
            .arg("--asdeps")
            .args(&pkgs)
            .status()
            .expect("Couldn't call pacman");
        match result.code() {
            Some(0) => succ(format!("Succesfully installed packages: {}", pkg)),
            Some(_) => err_unrec(format!("Couldn't install packages: {}", pkg)),
            None => err_unrec(format!("Couldn't install packages: {}", pkg)),
        };
    }
}

/*
//this function is used to install packages from a file
pub fn install_from_file(noconfirm: bool, as_dep: bool, file: &str) {
    let mut pkgs: Vec<&str> = Vec::new();
   // let mut file = File::open(file).expect("Couldn't open file");
    let mut contents = String::new();
    contents = std::fs::read_to_string(&file).expect("Couldn't read file");
    for line in contents.lines() {
        println!("{}", line);
        pkgs.push(line);
    }
    if as_dep == false {
        if noconfirm == true {
            let result = Command::new("pacman")
                .arg("-S")
                .arg("--noconfirm")
                .arg("--needed")
                .args(&pkgs)
                .status()
                .expect("Couldn't call pacman");
            match result.code() {
                Some(0) => succ(format!("Succesfully installed packages: {}", contents)),
                Some(_) => err_unrec(format!("Couldn't install packages: {}", contents)),
                None => err_unrec(format!("Couldn't install packages: {}", contents)),
            };
        } else {
            let result = Command::new("pacman")
                .arg("-S")
                .arg("--needed")
                .args(&pkgs)
                .status()
                .expect("Couldn't call pacman");
            match result.code() {
                Some(0) => succ(format!("Succesfully installed packages: {}", contents)),
                Some(_) => err_unrec(format!("Couldn't install packages: {}", contents)),
                None => err_unrec(format!("Couldn't install packages: {}", contents)),
            };
        }
    } else {
        let result = Command::new("pacman")
            .arg("-S")
            .arg("--noconfirm")
            .arg("--needed")
            .arg("--asdeps")
            .args(&pkgs)
            .status()
            .expect("Couldn't call pacman");
        match result.code() {
            Some(0) => succ(format!("Succesfully installed packages: {}", contents)),
            Some(_) => err_unrec(format!("Couldn't install packages: {}", contents)),
            None => err_unrec(format!("Couldn't install packages: {}", contents)),
        };
    }
}
*/