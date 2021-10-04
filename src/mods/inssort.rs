use crate::{clone, err_unrec, install, mods::strs::sec};
use std::process::{Command, Stdio};
use raur::SearchBy::Name;

pub fn inssort(noconfirm: bool, pkgs: Vec<String>) {
    let mut repo = vec![];
    let mut aur = vec![];
    for pkg in pkgs {
        let out = Command::new("pacman")
            .arg("-Ss")
            .arg(format!("^{}$", &pkg))
            .stdout(Stdio::null())
            .status()
            .expect("Something has gone wrong.");
        match out.code() {
            Some(0) => repo.push(pkg),
            Some(1) => aur.push(pkg),
            Some(_) => err_unrec(format!("Something has gone terribly wrong")),
            None => err_unrec(format!("Process terminated")),
        }
    }

    if repo.len() != 0 {
        sec(format!("Installing repo packages: {}", &repo.join(", ")));
        install(noconfirm, &repo.join(" "));
    }

    for a in aur {
        let sea = raur::search_by(format!("^{}$", a), Name).unwrap();
        if sea.len() == 0 {
            err_unrec(format!("No matching packages found"))
        } else {
            sec(format!("Installing AUR package: {}", a));
            clone(noconfirm, &a);
        }
    }
}
