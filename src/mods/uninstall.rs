use runas::Command;
use crate::mods::strs::{err_unrec, sec, succ};

pub fn uninstall(noconfirm: bool, pkg: &str) {
        sec(format!("Attempting to uninstall {}", pkg));
        if noconfirm == true {
            let result = Command::new("pacman").arg("-Rs").arg(&pkg).arg("--noconfirm").status();
            match result {
            Ok(_) => {
                succ(format!("Succesfully uninstalled {}", pkg))
                }
            Err(_) => {
                err_unrec(format!("Couldn't uninstall {}", pkg))
            }};
        } else {
            let result = Command::new("pacman").arg("-Rs").arg(&pkg).status();
            match result {
            Ok(_) => {
                succ(format!("Succesfully uninstalled {}", pkg))
            }
            Err(_) => {
                err_unrec(format!("Couldn't uninstall {}", pkg))
            }};
        }
}
