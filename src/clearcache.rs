use std::fs;

pub fn clearcache() {
    let path = format!("{}/.cache/ame/", std::env::var("HOME").unwrap());

    fs::remove_dir_all(&path).unwrap();
    fs::create_dir(&path).unwrap();
}