use crate::mods::strs::{err_unrec, succ};
use runas::Command;

pub fn install(noconfirm: bool, pkg: &str) {
    let pkgs: Vec<&str> = pkg.split(" ").collect();
    if noconfirm == true {
        let result = Command::new("pacman")
            .arg("-Sy")
            .arg("--noconfirm")
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
            .arg("-Sy")
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
