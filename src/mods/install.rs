use crate::mods::strs::{err_unrec, succ};
use runas::Command;

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
