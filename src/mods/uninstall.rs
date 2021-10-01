use runas::Command;
use crate::mods::strs::{err_unrec, sec, succ};

pub fn uninstall(noconfirm: bool, pkg: Vec<String>) {
        sec(format!("Attempting to uninstall packages: {}", &pkg.join(" ")));
        if noconfirm == true {
            let result = Command::new("pacman").arg("-Rs").args(&pkg).arg("--noconfirm").status();
            match result {
            Ok(_) => {
                succ(format!("Succesfully uninstalled packages: {}", &pkg.join(" ")))
                }
            Err(_) => {
                err_unrec(format!("Couldn't uninstall packages: {}", &pkg.join(" ")))
            }};
        } else {
            let result = Command::new("pacman").arg("-Rs").args(&pkg).status();
            match result {
            Ok(_) => {
                succ(format!("Succesfully uninstalled packages: {}", &pkg.join(" ")))
            }
            Err(_) => {
                err_unrec(format!("Couldn't uninstall packages: {}", &pkg.join(" ")))
            }};
        }
}
