use std::process::Command;


pub fn flatpak(pkg:&str) {
    let error = format!("Couldn't install {}", &pkg);
    Command::new("flatpak")
        .arg("install")
        .arg(&pkg)
        .status()
        .expect(&error);
}