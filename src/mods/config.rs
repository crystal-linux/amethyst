use toml;
use serde;
use std::{fs::File, io::prelude::*, env};


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



pub fn printconfig() {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open("config.toml").expect("Unable to open the Config file");
    let mut config = String::new();
    file.read_to_string(&mut config).expect("Unable to read the Config file");
    let configfile: General = toml::from_str(&config).unwrap();
    println!("\
General:
    Cache directory: {}

Backends:
    pacman support: {}
    aur support: {}
    flatpak support: {}
    snap support: {}

Pacman:
    noconfirm: {}

aur:
    Clone directory: {}", configfile.cache.unwrap(), configfile.backends.pacman.unwrap(), configfile.backends.aur.unwrap(), configfile.backends.flatpak.unwrap(), configfile.backends.snap.unwrap(), configfile.pacman.noconfirm.unwrap(), configfile.aur.clone_path.unwrap())
}