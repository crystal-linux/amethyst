mod mods;
use toml;
use serde;
use std::fs::File;
use std::io::prelude::*;
use mods::{clearcache::clearcache, clone::clone, help::help, install::install, search::{a_search, r_search}, uninstall::uninstall, upgrade::upgrade, flatpak::flatpak, snap::snap};
use std::{env, process::exit, process::Command};

#[derive(serde::Deserialize)]
struct General {
    cache: Option<String>,
    backends: Backends,
    pacman: Pacman,
    aur: AUR,
}

#[derive(serde::Deserialize)]
struct Backends {
    pacman: Option<bool>,
    flatpak: Option<bool>,
    snap: Option<bool>,
    aur: Option<bool>,
}

#[derive(serde::Deserialize)]
struct Pacman {
    noconfirm: Option<bool>,
}

#[derive(serde::Deserialize)]
struct AUR {
    clone_path: Option<String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open("config.toml").expect("Unable to open the Config file");
    let mut config = String::new();
    file.read_to_string(&mut config).expect("Unable to read the Config file");
    println!("{}", config);
    let configfile: General = toml::from_str(&config).unwrap();

    if args.len() <= 1 {
        help();
        exit(1);
    }
    let oper = &args[1];
    if oper == "-S" {
        for arg in env::args().skip(2) {
            if configfile.backends.pacman.unwrap() == true {
                let out = Command::new("pacman").arg("-Ss").arg(&arg).status().unwrap();
                if out.success() {
                    let configoption_noconfirm = configfile.pacman.noconfirm.unwrap();
                    install(configoption_noconfirm, &arg);
                } else {
                    if configfile.backends.aur.unwrap() == true {
                        clone(&arg);
                    } else {
                        println!("ERROR: the package wasn't found in the repos and aur support is disabled");
                        println!("Please enable aur support if you wish to check if this package exists in the aur");
                        exit(1);
                    }
                }
            } else if configfile.backends.aur.unwrap() == true {
                clone(&arg)
            } else {
                println!("ERROR: it seems like neither pacman, nor aur support is enabled!");
                println!("Please enable either one of those option and try again");
                exit(1);
            }
        } 
    } else if oper == "-R" {
        for arg in env::args().skip(2) {
            let configoption_noconfirm = configfile.pacman.noconfirm.unwrap();
            uninstall(configoption_noconfirm, &arg);
        }
    } else if oper == "-Syu" {
        let configoption_noconfirm = configfile.pacman.noconfirm.unwrap();
        upgrade(configoption_noconfirm);
    } else if oper == "-Ss" {
        for arg in env::args().skip(2) {
            r_search(&arg);
            a_search(&arg);
        }
    } else if oper == "-Sa" {
        for arg in env::args().skip(2) {
            a_search(&arg);
        }
    } else if oper == "-Sr" {
        for arg in env::args().skip(2) {
            r_search(&arg);
        }
    } else if oper == "-Cc" {
        clearcache();
    } else if oper == "-f" {
        if configfile.backends.flatpak.unwrap() == true {
            let b = std::path::Path::new("/usr/bin/flatpak").exists();
            if b == true {
                for arg in env::args().skip(2) {
                    flatpak(&arg);
                }
            } else {
                println!("ERROR: flatpak not found, please install flatpak and try again!");
                println!("If you do have flatpak installed, please open an issue on the ame github repo!");
                exit(1);
            }
        } else {
            println!("ERROR: flatpak support is disabled in your ame config!");
            println!("Enable flatpak support in your configuration and try again!");
            exit(1);
        }
    } else if oper == "-s" {
        if configfile.backends.snap.unwrap() == true {
            let b = std::path::Path::new("/usr/bin/snap").exists();
            if b == true {
                for arg in env::args().skip(2) {
                    snap(&arg)
                }
            } else {
                println!("ERROR: snap not found, please install snap and try again!");
                println!("If you do have snap installed, please open an issue on the ame github repo!");
                exit(1);
            }
        } else {
            println!("ERROR: snap support is disabled in your ame config!");
            println!("Enable snap support in your configuration and try again!");
            exit(1);
        }
    } else {
        help();
        exit(0);
    }
}
