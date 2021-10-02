use crate::mods::strs::err_rec;
use std::fs;

pub fn clearcache() {
    let path = format!("{}/.cache/ame/", std::env::var("HOME").unwrap());

    err_rec(format!("Clearing cache"));

    fs::remove_dir_all(&path).unwrap();
    fs::create_dir(&path).unwrap();
}
