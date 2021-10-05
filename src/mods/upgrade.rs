use crate::mods::strs::{err_unrec, inf, sec, succ};
use runas::Command;
use std::{env, fs};

pub fn upgrade(noconfirm: bool) -> std::io::Result<()>{
    let homepath = std::env::var("HOME").unwrap();
    let cachedir = format!("/{}/.cache/ame/", homepath);
    let cache_exists = std::path::Path::new(&format!("/{}/.cache/ame/", homepath)).is_dir();
    if cache_exists == false {
        fs::create_dir_all(&cachedir)?;
    }
    sec(format!("Performing system upgrade"));
    if noconfirm == true {
        let result = Command::new("pacman")
            .arg("-Syu")
            .arg("--noconfirm")
            .status()
            .expect("Couldn't call pacman");
        match result.code() {
            Some(0) => succ(format!("All repo packages upgraded")),
            Some(_) => err_unrec(format!("Couldn't upgrade packages")),
            None => err_unrec(format!("Couldn't upgrade packages")),
        };
    } else {
        let result = Command::new("pacman")
            .arg("-Syu")
            .status()
            .expect("Couldn't call pacman");
        match result.code() {
            Some(0) => succ(format!("All repo packages upgraded")),
            Some(_) => err_unrec(format!("Couldn't upgrade packages")),
            None => err_unrec(format!("Couldn't upgrade packages")),
        };
    }

    for file in std::fs::read_dir(&cachedir).unwrap() {
        let dir = &file.unwrap().path();
        let output = std::process::Command::new("git")
            .arg("pull")
            .output()
            .unwrap();
        let update_available = String::from_utf8(output.stdout).unwrap();

        let cd_result = env::set_current_dir(&dir);
        match cd_result {
            Ok(_) => inf(format!("Entered AUR package directory to pull changes")),
            Err(_) => err_unrec(format!(
                "Could not enter AUR package directory to pull changes"
            )),
        }

        if update_available != "Already up to date." {
            let path_as_str = &dir.display().to_string();
            let pkg: Vec<&str> = path_as_str.split("/").collect();

            inf(format!("{} is up to date", pkg[pkg.len() - 1]));
        } else {
            let cd2_result = env::set_current_dir(&dir);
            match cd2_result {
                Ok(_) => inf(format!(
                    "Entering AUR package directory to install new version"
                )),
                Err(_) => err_unrec(format!(
                    "Couldn't enter AUR package directory to install new version"
                )),
            }

            let makepkg_result = std::process::Command::new("makepkg")
                .arg("-si")
                .status()
                .expect("Couldn't call makepkg");
            match makepkg_result.code() {
                Some(0) => succ(format!("New AUR package version installed")),
                Some(_) => err_unrec(format!("Couldn't install new AUR package version")),
                None => err_unrec(format!("Couldn't install new AUR package version")),
            };
        }
    }
    Ok(())
}
