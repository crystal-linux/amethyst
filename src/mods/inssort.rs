use crate::{clone, install, err_unrec, mods::strs::sec};
use std::process::{Stdio, Command};

pub fn inssort(noconfirm: bool, pkgs: Vec<String>) {
    let mut repo = vec![];
    let mut aur = vec![];
    for pkg in pkgs {
        let out = Command::new("pacman")
                        .arg("-Ss")
                        .arg(&pkg)
                        .stdout(Stdio::null())
                        .status()
                        .expect("Something has gone wrong.");
        match out.code() {
        Some(0) => {
            repo.push(pkg)
        }
        Some(1) => {
            aur.push(pkg)
        }
        Some(_) => {
            err_unrec(format!("Something has gone terribly wrong"))
        }
        None => {
            err_unrec(format!("Process terminated"))
        }}}

    if repo.len() != 0 {
        sec(format!("Installing repo packages: {}", &repo.join(", ")));
        if noconfirm == true {
            install(true, &repo.join(" "));
        } else {
           install(false, &repo.join(" "));
       }
    }

    for a in aur {
       sec(format!("Installing AUR package: {}", a));
       if noconfirm == true {
           clone(true, &a);
       } else {
           clone(false, &a);
        }
    }
}
