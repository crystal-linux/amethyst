use toml;
use serde;
use std::{fs, fs::File, io::prelude::*};


#[derive(serde::Deserialize)]
struct General {
    cache: Option<String>,
    backends: Backends,
    pacman: Pacman,
}

#[derive(serde::Deserialize)]
struct Backends {
    pacman: Option<bool>,
    flatpak: Option<bool>,
    aur: Option<bool>,
}

#[derive(serde::Deserialize)]
struct Pacman {
    noconfirm: Option<bool>,
}



pub fn printconfig() {
    let mut confile = File::open("/etc/ame.toml").expect("Unable to open the Config file, did you delete ame.toml from /etc/??");
    let mut config = String::new();
    let defaultconfig = format!(r#"
        cache = "{}/.cache/ame"  

        [backends]
        pacman = true
        flatpak = true
        aur = true

        [pacman]
        noconfirm = false
    "#, std::env::var("HOME").unwrap());
    let mut configfile: General = toml::from_str(&defaultconfig).unwrap();
    if fs::read_to_string("/etc/ame.toml").expect("unable to open config file! (/etc/ame.toml)") != "" {
        confile.read_to_string(&mut config).expect("Unable to read the Config file (/etc/ame.toml)");
        configfile = toml::from_str(&config).unwrap();
    }
    println!("\
General:
    Cache directory: {}

Backends:
    pacman support: {}
    aur support: {}
    flatpak support: {}

Pacman:
    noconfirm: {}
", configfile.cache.unwrap(), configfile.backends.pacman.unwrap(), configfile.backends.aur.unwrap(), configfile.backends.flatpak.unwrap(), configfile.pacman.noconfirm.unwrap());
}