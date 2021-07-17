use std::process::Command;

pub fn upgrade() {
    let errstr = format!("Something happened");
    Command::new("pacman")
        .arg("-Syu")
        .output()
        .expect(&errstr);
}