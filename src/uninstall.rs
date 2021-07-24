use std::process::Command;

pub fn uninstall(pkg: &str) {
    let errstr = format!("Could not remove package {}", pkg);
    Command::new("sudo")
        .arg("pacman")
        .arg("-R")
        .arg(&pkg)
        .spawn()
        //.status() TODO: for some reason cant use both .spawn and .status at the same time, need fix
        .expect(&errstr);
}