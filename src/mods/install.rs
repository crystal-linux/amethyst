use runas::Command;
use crate::mods::strs::{inf, err_unrec};

pub fn install(noconfirm: bool, pkg: &str) {
    let pkgs: Vec<&str> = pkg.split(" ").collect();
    if noconfirm == true {
        let result = Command::new("pacman").arg("-Sy").arg("--noconfirm").args(&pkgs).status();
        match result {
        Ok(_) => {
            inf(format!("Succesfully installed packages: {}", pkg))
        }
        Err(_) => {
            err_unrec(format!("Couldn't install packages: {}", pkg))
        }};
    } else {
        let result = Command::new("pacman").arg("-Sy").args(&pkgs).status();
        match result {
        Ok(_) => {
            inf(format!("Succesfully installed packages: {}", pkg))
        }
        Err(_) => {
            err_unrec(format!("Couldn't install packages: {}", pkg))
        }};
    }
}
