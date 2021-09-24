use runas::Command;
use crate::mods::strs::{inf, err_unrec};

pub fn uninstall(pkg: &str) {
        inf(format!("Attempting to uninstall {}", pkg));
        let result = Command::new("pacman").arg("-Rs").arg(&pkg).status();
        match result {
        Ok(_) => {
            println!("")
        }
        Err(_) => {
            err_unrec(format!("Couldn't uninstall {}", pkg))
        }};
}
