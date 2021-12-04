use crate::mods::{
    database::rem_pkg,
    strs::{err_rec, err_unrec, sec, succ},
};
use runas::Command;
use std::{fs, path::Path};

pub fn uninstall(noconfirm: bool, pkgs: Vec<String>) {
    // uninstall a package
    sec(format!(
        "Attempting to uninstall packages: {}",
        &pkgs.join(" ")
    ));

    let important = [
        "base",
        "linux",
        "linux-firmware",
        "systemd-sysvcompat",
        "networkmanager",
        "man-db",
        "man-pages",
        "texinfo",
        "sudo",
        "curl",
        "archlinux-keyring",
        "btrfs-progs",
        "timeshift",
        "timeshift-autosnap",
    ];

    let mut overrides: Vec<String> = Vec::new();
    if Path::new("/etc/ame/overrides.conf").exists() {
        overrides = fs::read_to_string("/etc/ame/overrides.conf")
            .expect("Failed to read overrides.conf")
            .lines()
            .map(|s| s.to_string())
            .collect();
    }

    let mut matches: Vec<String> = Vec::new();
    for pkg in pkgs.iter() {
        for imp in important.iter() {
            if pkg == imp && !overrides.contains(pkg) {
                matches.push(pkg.to_string());
            }
        }
    }
    err_unrec(format!("The action you called for tries to uninstall packages: {} . This is disallowed by default as these are important system packages. If you fully know what you are doing and would like to uninstall these, please create an override in /etc/ame/overrides.conf.", matches.join(" ")));

    if noconfirm {
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

pub fn uninstall_from_file(noconfirm: bool, file: &str) {
    // uninstall a package from a list of packages
    let mut pkgs: Vec<String> = Vec::new();
    let contents = std::fs::read_to_string(&file).expect("Couldn't read file");
    for line in contents.lines() {
        pkgs.push(line.to_string());
    }
    sec(format!(
        "Attempting to uninstall packages: {}",
        &pkgs.join(" ")
    ));
    if noconfirm {
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
