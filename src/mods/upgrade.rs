use crate::{
    err_rec, err_unrec, inf, inssort, mods::database::get_value, mods::strs::prompt,
    mods::strs::sec, mods::strs::succ, uninstall,
};
use runas::Command;
use std::{env, fs, path::Path};
use toml;

fn uninstall_make_depend(pkg: &str) {
    // uninstall make depends installed by ame itself
    let make_depends = raur::info(&[&pkg]).unwrap()[0].make_depends.clone();

    if !make_depends.is_empty() {
        inf(format!(
            "{} installed following make dependencies: {}",
            pkg,
            make_depends.join(", ")
        ));
        let remove = prompt("Would you like to remove them?".to_string());
        if remove {
            uninstall(true, make_depends);
        }
    }
    succ(format!("Succesfully upgraded {}", pkg));
}

pub fn upgrade(noconfirm: bool) {
    // upgrade all packages
    let homepath = env::var("HOME").unwrap();
    let cachedir = format!("/{}/.cache/ame/", homepath);
    let cache_exists = Path::new(&format!("/{}/.cache/ame/", homepath)).is_dir();
    let file = format!("{}/.local/ame/aurPkgs.db", env::var("HOME").unwrap());
    let database = String::new();
    if Path::new(&file).exists() {
        let _db = fs::read_to_string(&file).expect("Can't Open Database");
    } else {
        let _cdar = fs::create_dir_all(format!("/{}/.local/ame/", homepath));
        match _cdar {
            Ok(_) => inf("Created cache directory (previously missing)".to_string()),
            Err(_) => err_unrec("Couldn't create cache directory".to_string()),
        }
        err_rec(String::from("Database wasn't found, creating new one"));
        let _dbfile = fs::File::create(&file);
        let _db = String::new();
    }
    let db_parsed = database.parse::<toml::Value>().expect("Invalid Database");

    if !cache_exists {
        let cachecreate = fs::create_dir_all(&cachedir);
        match cachecreate {
            Ok(_) => inf("Creating cachedir. (didn't exist previously)".to_string()),
            Err(_) => err_unrec("Couldn't create cachedir".to_string()),
        }
    }
    sec("Performing system upgrade".to_string());
    if noconfirm {
        let result = Command::new("pacman")
            .arg("-Syu")
            .arg("--noconfirm")
            .status()
            .expect("Couldn't call pacman");
        match result.code() {
            Some(0) => succ("All repo packages upgraded".to_string()),
            Some(_) => err_unrec("Couldn't upgrade packages".to_string()),
            None => err_unrec("Couldn't upgrade packages".to_string()),
        };
    } else {
        let result = Command::new("pacman")
            .arg("-Syu")
            .status()
            .expect("Couldn't call pacman");
        match result.code() {
            Some(0) => succ("All repo packages upgraded".to_string()),
            Some(_) => err_unrec("Couldn't upgrade packages".to_string()),
            None => err_unrec("Couldn't upgrade packages".to_string()),
        };
    }

    if let Some(entry) = db_parsed.as_table() {
        for (key, _value) in &*entry {
            let results = raur::search(key.to_string());
            if let Ok(res) = results {
                let url = format!("https://aur.archlinux.org/{}.git", key);
                let package = raur::info(&[key]).unwrap();
                let version = get_value(key, "version");
                if res[0].version.contains(&version) {
                    let keydir = format!("{}{}", &cachedir, &key);
                    if Path::new(&keydir).is_dir() {
                        let cd_result = env::set_current_dir(&keydir);
                        match cd_result {
                            Ok(_) => inf("Entered package directory".to_string()),
                            Err(_) => err_unrec("Could not enter package directory".to_string()),
                        }
                        inssort(true, true, package[0].depends.clone());

                        sec(format!("Installing {} ...", &key));
                        let install_result = std::process::Command::new("makepkg")
                            .arg("-si")
                            .arg("--noconfirm")
                            .arg("--needed")
                            .status();
                        match install_result {
                            Ok(_) => {
                                uninstall_make_depend(key);
                            }
                            Err(_) => {
                                err_unrec(format!("Couldn't install {}", &key));
                            }
                        };

                        sec(format!("Installing {} ...", &key));
                        let install_result = std::process::Command::new("makepkg")
                            .arg("-si")
                            .arg("--needed")
                            .status()
                            .expect("Couldn't call makepkg");
                        match install_result.code() {
                            Some(0) => {
                                uninstall_make_depend(key);
                            }
                            Some(_) => {
                                err_unrec(format!("Couldn't install {}", &key));
                            }
                            None => {
                                err_unrec(format!("Couldn't install {}", &key));
                            }
                        };
                    } else {
                        inf(format!("Cloning {} ...", &key));

                        if Path::new(&keydir).is_dir() {
                            let rm_result = fs::remove_dir_all(&keydir);
                            match rm_result {
                                    Ok(_) => inf(format!(
                                        "Package path for {} already found. Removing to reinstall",
                                        &key
                                    )),
                                    Err(_) => err_unrec(format!(
                                        "Package path for {} already found, but could not remove to reinstall",
                                        &key
                                    )),
                                }
                        }

                        let dir_result = fs::create_dir(&keydir);
                        match dir_result {
                            Ok(_) => inf(format!("Created package directory for {}", &key)),
                            Err(_) => {
                                err_unrec(format!("Couldn't create package directory for {}", &key))
                            }
                        }

                        let cd_result = env::set_current_dir(&keydir);
                        match cd_result {
                            Ok(_) => inf("Entered package directory".to_string()),
                            Err(_) => err_unrec("Could not enter package directory".to_string()),
                        }

                        inssort(true, true, package[0].depends.clone());

                        let clone = std::process::Command::new("git")
                            .arg("clone")
                            .arg(&url)
                            .arg(&keydir)
                            .status()
                            .expect("Couldn't clone repo");
                        match clone.code() {
                            Some(0) => {
                                inf(format!("Cloning {} into package directory", &key));
                            }
                            Some(_) => {
                                err_unrec(format!("Failed cloning {} into package directory", &key))
                            }
                            _ => {
                                err_unrec(format!("Failed cloning {} into package directory", &key))
                            }
                        }
                    }

                    sec(format!("Installing {} ...", &key));
                    let install_result = std::process::Command::new("makepkg")
                        .arg("-si")
                        .arg("--noconfirm")
                        .arg("--needed")
                        .status();
                    match install_result {
                        Ok(_) => {
                            uninstall_make_depend(key);
                        }
                        Err(_) => {
                            err_unrec(format!("Couldn't install {}", &key));
                        }
                    };
                    sec(format!("Installing {} ...", &key));
                    let install_result = std::process::Command::new("makepkg")
                        .arg("-si")
                        .arg("--needed")
                        .status()
                        .expect("Couldn't call makepkg");
                    match install_result.code() {
                        Some(0) => {
                            uninstall_make_depend(key);
                        }
                        Some(_) => {
                            err_unrec(format!("Couldn't install {}", &key));
                        }
                        None => {
                            err_unrec(format!("Couldn't install {}", &key));
                        }
                    };
                } else {
                    inf(format!("Package {} already up to date", &key));
                }
            }
        }
    }
}
