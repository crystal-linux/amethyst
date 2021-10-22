use crate::{clone, err_unrec, install, mods::strs::sec};
use std::process::{Command, Stdio};
use regex::Regex;

pub fn inssort(noconfirm: bool, as_dep: bool, pkgs: Vec<String>) {
    let mut repo = vec![];
    let mut aur = vec![];
    let re = Regex::new(r"(\S+)((?:>=|<=)\S+$)").unwrap();
    let reg = Regex::new(r"((?:>=|<=)\S+$)").unwrap();
    for pkg in pkgs {
        let caps = re.captures(&pkg);
        match caps {
            Some(_) => {
                let out = Command::new("pacman")
                    .arg("-Ss")
                    .arg(format!(
                        "^{}$",
                        caps.unwrap().get(1).map_or("", |m| m.as_str())
                    ))
                    .stdout(Stdio::null())
                    .status()
                    .expect("Something has gone wrong.");
                match out.code() {
                    Some(0) => repo.push(reg.replace_all(&pkg, "").to_string()),
                    Some(1) => aur.push(pkg),
                    Some(_) => err_unrec(format!("Something has gone terribly wrong")),
                    None => err_unrec(format!("Process terminated")),
                }
            }
            None => {
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
        }
    }
    if as_dep == false {
        if repo.len() != 0 {
            sec(format!("Installing repo packages: {}", &repo.join(", ")));
            install(noconfirm, false, &repo.join(" "));
        }

        for a in aur {
            sec(format!("Couldn't find {} in repos. Searching AUR", a));
            clone(noconfirm, false, &a);
        }
    } else {
        if repo.len() != 0 {
            sec(format!("Installing repo packages: {}", &repo.join(", ")));
            install(noconfirm, true,&repo.join(" "));
        }

        for a in aur {
            sec(format!("Couldn't find {} in repos. Searching AUR", a));
            clone(noconfirm, true, &a);
        }
    }
}
