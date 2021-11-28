use crate::mods::strs::{err_unrec, sec, succ};
use runas::Command;

pub fn update() {
    // update the repositories
    sec("Syncing package repos".to_string());

    let result = Command::new("pacman")
        .arg("-Sy")
        .status()
        .expect("Couldn't call pacman");
    match result.code() {
        Some(0) => succ("Repos succesfully synced".to_string()),
        Some(_) => err_unrec("Couldn't sync package repos".to_string()),
        None => err_unrec("Couldn't sync package repos".to_string()),
    }
}
