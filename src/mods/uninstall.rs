use std::process::Command;

pub fn uninstall(pkg: &str) {
    let errstr = format!("Could not remove package {}", pkg);
    Command::new("sudo")
        .arg("pacman")
        .arg("-R")
        .arg("--noconfirm")
        .arg(&pkg)
        .status()
        .expect(&errstr);
}
