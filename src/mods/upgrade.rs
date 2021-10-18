use crate::{
    err_rec, err_unrec, inf, inssort, mods::strs::prompt, mods::strs::sec, mods::strs::succ, uninstall, mods::database::get_value,
};
use git2::Repository;
use runas::Command;
use std::{env, fs, path::Path};
use toml;

fn uninstall_make_depend(pkg: &str) {
    let make_depends = raur::info(&[&pkg]).unwrap()[0].make_depends.clone();

    if make_depends.len() != 0 {
        inf(format!(
            "{} installed following make dependencies: {}",
            pkg,
            make_depends.join(", ")
        ));
        let remove = prompt(format!("Would you like to remove them?"));
        if remove == true {
            uninstall(true, make_depends);
        }
    }
    succ(format!("Succesfully upgraded {}", pkg));
}

pub fn upgrade(noconfirm: bool) {
    let homepath = std::env::var("HOME").unwrap();
    let cachedir = format!("/{}/.cache/ame/", homepath);
    let cache_exists = std::path::Path::new(&format!("/{}/.cache/ame/", homepath)).is_dir();
    let file = format!("{}/.local/ame/aurPkgs.db", std::env::var("HOME").unwrap());
    let database = String::new();
    if std::path::Path::new(&file).exists() {
        let _db = std::fs::read_to_string(&file).expect("Can't Open Database");
    } else {
        let _cdar = fs::create_dir_all(format!("/{}/.local/ame/", homepath));
        match _cdar {
            Ok(_) => {
                inf(format!("Created cache directory (previously missing)"))
            }
            Err(_) => {
                err_unrec(format!("Couldn't create cache directory"))
            }
        }
        err_rec(String::from("Database wasn't found, creating new one"));
        let _dbfile = std::fs::File::create(&file);
        let _db = String::new();
    }
    let db_parsed = database.parse::<toml::Value>().expect("Invalid Database");

    if cache_exists == false {
        let cachecreate = fs::create_dir_all(&cachedir);
        match cachecreate {
            Ok(_) => inf(format!("Creating cachedir. (didn't exist previously)")),
            Err(_) => err_unrec(format!("Couldn't create cachedir")),
        }
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

    for entry in db_parsed.as_table() {
        for (key, _value) in &*entry {
            let results = raur::search(format!("{}", key));
            for res in results {
                let url = format!("https://aur.archlinux.org/{}.git", key);
                let package = raur::info(&[key]).unwrap();
                let version = get_value(&key, "version");
                if res[0].version.contains(&version) {
                    let keydir = format!("{}{}", &cachedir, &key);
                    if std::path::Path::new(&keydir).is_dir() {
                        let cd_result = env::set_current_dir(&keydir);
                        match cd_result {
                            Ok(_) => inf(format!("Entered package directory")),
                            Err(_) => err_unrec(format!("Could not enter package directory")),
                        }
                        inssort(true, true,package[0].depends.clone());

                        sec(format!("Installing {} ...", &key));
                        let install_result = std::process::Command::new("makepkg")
                            .arg("-si")
                            .arg("--noconfirm")
                            .arg("--needed")
                            .status();
                        match install_result {
                            Ok(_) => {
                                uninstall_make_depend(&key);
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
                                uninstall_make_depend(&key);
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
                            Ok(_) => inf(format!("Entered package directory")),
                            Err(_) => err_unrec(format!("Could not enter package directory")),
                        }

                        inssort(true, true, package[0].depends.clone());

                        let clone = Repository::clone(&url, Path::new(&keydir));
                        match clone {
                            Ok(_) => {
                                inf(format!("Cloning {} into package directory", &key));
                            }
                            Err(_) => {
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
                            uninstall_make_depend(&key);
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
                            uninstall_make_depend(&key);
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
