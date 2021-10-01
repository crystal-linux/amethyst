use runas::Command;
use crate::mods::strs::{err_unrec, sec, succ, err_rec};
use std::{fs, path::Path};

pub fn uninstall(noconfirm: bool, pkgs: Vec<String>) {
        sec(format!("Attempting to uninstall packages: {}", &pkgs.join(" ")));
        if noconfirm == true {
            let result = Command::new("pacman").arg("-Rs").args(&pkgs).arg("--noconfirm").status().expect("Couldn't call pacman");
            match result.code() {
            Some(0) => {
                succ(format!("Succesfully uninstalled packages: {}", &pkgs.join(" ")))
            }
            Some(_) => {
                err_rec(format!("Couldn't uninstall packages: {}", &pkgs.join(" ")))
            }
            None =>{
                err_rec(format!("Couldn't uninstall packages: {}", &pkgs.join(" ")))
            }};
        } else {
            let result = Command::new("pacman").arg("-Rs").args(&pkgs).status().expect("Couldn't call pacman");
            match result.code() {
            Some(0) => {
                succ(format!("Succesfully uninstalled packages: {}", &pkgs.join(" ")))
            }
            Some(_) => {
                err_rec(format!("Couldn't uninstall packages: {}", &pkgs.join(" ")))
            }
            None =>{
                err_rec(format!("Couldn't uninstall packages: {}", &pkgs.join(" ")))
            }};
        }
        for pkg in &pkgs {
            let pkgdir = format!("{}/.cache/ame/{}", std::env::var("HOME").unwrap(), pkg);
            let path = Path::new(&pkgdir);
            if path.is_dir() {
                let rm_result = fs::remove_dir_all(&path);
                match rm_result {
                Ok(_) => {
                    succ(format!("Removed AUR cache directory for {}", pkg))
                }
                Err(_) => {
                    err_unrec(format!("Failed to remove AUR cache directory for {}", pkg))
                }};
            }
        }
}
