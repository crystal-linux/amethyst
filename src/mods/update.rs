use runas::Command;
use crate::mods::strs::{inf, err_unrec};

pub fn update() {
    inf(format!("Syncing package repos"));

    let result = Command::new("pacman")
                         .arg("-Sy")
                         .status();
    match result {
    Ok(_) => {
        inf(format!("Repos succesfully synced"))
    }
    Err(_) => {
        err_unrec(format!("Couldn't sync package repos (how?)"))
    }}
}
