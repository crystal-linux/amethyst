use runas::Command;
use crate::mods::strs::{inf, err_unrec};

pub fn install(pkg: &str) {
    let result = Command::new("pacman").arg("-Sy").arg(&pkg).status();
    match result {
    Ok(_) => {
        inf(format!("Succesfully installed {}", pkg))
    }
    Err(_) => {
        err_unrec(format!("Couldn't install {}", pkg))
    }};
}
