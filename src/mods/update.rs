use runas::Command;
use crate::mods::strs::{err_unrec, sec, succ};

pub fn update() {
    sec(format!("Syncing package repos"));

    let result = Command::new("pacman")
                         .arg("-Sy")
                         .status()
                         .expect("Couldn't call pacman");
    match result.code() {
    Some(0) => {
        succ(format!("Repos succesfully synced"))
    }
    Some(_) => {
        err_unrec(format!("Couldn't sync package repos"))
    }
    None => {
        err_unrec(format!("Couldn't sync package repos"))
    }}
}
