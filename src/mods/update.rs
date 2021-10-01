use runas::Command;
use crate::mods::strs::{err_unrec, sec, succ};

pub fn update() {
    sec(format!("Syncing package repos"));

    let result = Command::new("pacman")
                         .arg("-Sy")
                         .status();
    match result {
    Ok(_) => {
        succ(format!("Repos succesfully synced"))
    }
    Err(_) => {
        err_unrec(format!("Couldn't sync package repos (how?)"))
    }}
}
