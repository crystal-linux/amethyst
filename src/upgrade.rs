use std::process::Command;

pub fn upgrade() {
    let errstr = format!("Something happened");
    Command::new("pacman")
        .arg("-Syu")
        .spawn()
        //.status() TODO: for some reason cant use both .spawn and .status at the same time, need fix
        .expect(&errstr);
}