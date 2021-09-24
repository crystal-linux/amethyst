use std::fs;
use crate::mods::strs::inf;

pub fn clearcache() {
    let path = format!("{}/.cache/ame/", std::env::var("HOME").unwrap());

    inf(format!("Clearing cache"));

    fs::remove_dir_all(&path).unwrap();
    fs::create_dir(&path).unwrap();
}
