use crate::mods::{
    database::rem_pkg,
    strs::{err_rec, err_unrec, sec, succ},
};
use runas::Command;
use std::{fs, path::Path};

pub fn uninstall(noconfirm: bool, pkgs: Vec<String>) { // uninstall a package
    sec(format!(
        "Attempting to uninstall packages: {}",
        &pkgs.join(" ")
    ));
    if noconfirm == true {
        let result = Command::new("pacman")
            .arg("-Ru")
            .args(&pkgs)
            .arg("--noconfirm")
            .status()
            .expect("Couldn't call pacman");
        match result.code() {
            Some(0) => {
                succ(format!(
                    "Succesfully uninstalled packages: {}",
                    &pkgs.join(" ")
                ));
                rem_pkg(&pkgs);
            }
            Some(_) => err_rec(format!("Couldn't uninstall packages: {}", &pkgs.join(" "))),
            None => err_rec(format!("Couldn't uninstall packages: {}", &pkgs.join(" "))),
        };
    } else {
        let result = Command::new("pacman")
            .arg("-Ru")
            .args(&pkgs)
            .status()
            .expect("Couldn't call pacman");
        match result.code() {
            Some(0) => {
                succ(format!(
                    "Succesfully uninstalled packages: {}",
                    &pkgs.join(" ")
                ));
                rem_pkg(&pkgs);
            }
            Some(_) => err_rec(format!("Couldn't uninstall packages: {}", &pkgs.join(" "))),
            None => err_rec(format!("Couldn't uninstall packages: {}", &pkgs.join(" "))),
        };
    }
    for pkg in &pkgs {
        let pkgdir = format!("{}/.cache/ame/{}", std::env::var("HOME").unwrap(), pkg);
        let path = Path::new(&pkgdir);
        if path.is_dir() {
            let rm_result = fs::remove_dir_all(&path);
            match rm_result {
                Ok(_) => succ(format!("Removed AUR cache directory for {}", pkg)),
                Err(_) => err_unrec(format!("Failed to remove AUR cache directory for {}", pkg)),
            };
        }
    }
}

pub fn uninstall_from_file(noconfirm: bool, file: &str) { // uninstall a package from a list of packages
    let mut pkgs: Vec<String> = Vec::new();
    let contents = std::fs::read_to_string(&file).expect("Couldn't read file");
    for line in contents.lines() {
        pkgs.push(line.to_string());
    }
    sec(format!(
        "Attempting to uninstall packages: {}",
        &pkgs.join(" ")
    ));
    if noconfirm == true {
        let result = Command::new("pacman")
            .arg("-Ru")
            .args(&pkgs)
            .arg("--noconfirm")
            .status()
            .expect("Couldn't call pacman");
        match result.code() {
            Some(0) => {
                succ(format!(
                    "Succesfully uninstalled packages: {}",
                    &pkgs.join(" ")
                ));
                rem_pkg(&pkgs);
            }
            Some(_) => err_rec(format!("Couldn't uninstall packages: {}", &pkgs.join(" "))),
            None => err_rec(format!("Couldn't uninstall packages: {}", &pkgs.join(" "))),
        };
    } else {
        let result = Command::new("pacman")
            .arg("-Ru")
            .args(&pkgs)
            .status()
            .expect("Couldn't call pacman");
        match result.code() {
            Some(0) => {
                succ(format!(
                    "Succesfully uninstalled packages: {}",
                    &pkgs.join(" ")
                ));
                rem_pkg(&pkgs);
            }
            Some(_) => err_rec(format!("Couldn't uninstall packages: {}", &pkgs.join(" "))),
            None => err_rec(format!("Couldn't uninstall packages: {}", &pkgs.join(" "))),
        };
    }
    for pkg in &pkgs {
        let pkgdir = format!("{}/.cache/ame/{}", std::env::var("HOME").unwrap(), pkg);
        let path = Path::new(&pkgdir);
        if path.is_dir() {
            let rm_result = fs::remove_dir_all(&path);
            match rm_result {
                Ok(_) => succ(format!("Removed AUR cache directory for {}", pkg)),
                Err(_) => err_unrec(format!("Failed to remove AUR cache directory for {}", pkg)),
            };
        }
    }
}