mod mods;
use toml;
use serde;
use mods::{clearcache::clearcache, clone::clone, help::help, install::install, search::{a_search, r_search}, uninstall::uninstall, upgrade::upgrade, flatpak::flatpak};
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

    let configfile: General = toml::from_str(r#"
        cache = "/home/user/.cache/ame"    

        [backends]
        pacman = true
        flatpak = false
        snap = false
        aur = true

        [pacman]
        noconfirm = false

        [aur]
        clone_path = "/home/user/.cache/ame"
    "#).unwrap();

    if args.len() <= 1 {
        help();
        exit(1);
    }
    let oper = &args[1];
    if oper == "-S" {
        for arg in env::args().skip(2) {
            let out = Command::new("pacman").arg("-Ss").arg(&arg).status().unwrap();
            if out.success() {
                install(&arg);
            } else {
                clone(&arg);
            }
        }
    } else if oper == "-R" {
        for arg in env::args().skip(2) {
            uninstall(&arg);
        }
    } else if oper == "-Syu" {
        upgrade();
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
            }
        } else {
            println!("ERROR: flatpak support is disabled in your ame config!");
            println!("Enable flatpak support in your configuration and try again!");
        }
    } else {
        help();
        exit(0);
    }
}
