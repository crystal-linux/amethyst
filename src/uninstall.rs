use std::process::Command;

pub fn uninstall(pkg: &str) {
    let errstr = format!("Could not remove package {}", pkg);
    Command::new("pacman")
        .arg("-R")
        .arg(&pkg)
        .output()
        .expect(&errstr);
}
