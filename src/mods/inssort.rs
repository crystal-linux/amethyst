use crate::{clone, err_unrec, install, mods::strs::sec, mods::rpc::*};
use regex::Regex;
use std::process::{Command, Stdio};

pub fn inssort(noconfirm: bool, as_dep: bool, pkgs: Vec<String>) {
    // TODO: understand what the fuck is actually going on here
    let mut repo = vec![];
    let mut aur = vec![];
    let re = Regex::new(r"(\S+)((?:>=|<=|>|<)\S+$)").unwrap();
    let reg = Regex::new(r"((?:>=|<=|>|<)\S+$)").unwrap();
    for pkg in pkgs {
        match pkg.contains('/') {
            true => match pkg.split('/').collect::<Vec<&str>>()[0] == "aur" {
                true => {
                    aur.push(pkg.split('/').collect::<Vec<&str>>()[1].to_string());
                }
                false => {
                    let out = Command::new("bash")
                        .arg("-c")
                        .arg(format!(
                            "pacman -Sl {} | grep {}",
                            pkg.split('/').collect::<Vec<&str>>()[0],
                            pkg.split('/').collect::<Vec<&str>>()[1]
                        ))
                        .stdout(Stdio::null())
                        .status()
                        .expect("Something has gone wrong.");
                    match out.code() {
                        Some(0) => repo.push(reg.replace_all(&pkg, "").to_string()),
                        Some(1) => err_unrec(format!(
                            "Package {} not found in repository {}",
                            pkg.split('/').collect::<Vec<&str>>()[1],
                            pkg.split('/').collect::<Vec<&str>>()[0]
                        )),
                        Some(_) => err_unrec("Something has gone terribly wrong".to_string()),
                        None => err_unrec("Process terminated".to_string()),
                    }
                }
            },
            false => {
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
                            Some(_) => err_unrec("Something has gone terribly wrong".to_string()),
                            None => err_unrec("Process terminated".to_string()),
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
                            Some(_) => err_unrec("Something has gone terribly wrong".to_string()),
                            None => err_unrec("Process terminated".to_string()),
                        }
                    }
                }
            }
        }
    }
    if !as_dep {
        if !repo.is_empty() {
            sec(format!("Installing repo packages: {}", &repo.join(", ")));
            install(noconfirm, false, &repo.join(" "));
        }

        for a in aur {
            sec(format!("Couldn't find {} in repos. Searching AUR", a));
            let md = &rpcinfo(&a).make_depends;
            inssort(noconfirm, true, md.to_vec());
            clone(noconfirm, false, &a);
        }
    } else {
        if !repo.is_empty() {
            sec(format!("Installing repo packages: {}", &repo.join(", ")));
            install(noconfirm, true, &repo.join(" "));
        }

        for a in aur {
            sec(format!("Couldn't find {} in repos. Searching AUR", a));
            let md = &rpcinfo(&a).make_depends;
            inssort(noconfirm, true, md.to_vec());
            clone(noconfirm, true, &a);
        }
    }
}

pub fn inssort_from_file(noconfirm: bool, as_dep: bool, file: &str) {
    // same thing as above but with a list of packages from a file
    let mut pkgs: Vec<String> = Vec::new();
    let contents = std::fs::read_to_string(&file).expect("Couldn't read file");
    for line in contents.lines() {
        pkgs.push(line.to_string());
    }
    let mut repo = vec![];
    let mut aur = vec![];
    let re = Regex::new(r"(\S+)((?:>=|<=)\S+$)").unwrap();
    let reg = Regex::new(r"((?:>=|<=)\S+$)").unwrap();
    for pkg in pkgs {
        match pkg.contains('/') {
            true => match pkg.split('/').collect::<Vec<&str>>()[0] == "aur" {
                true => {
                    aur.push(pkg.split('/').collect::<Vec<&str>>()[1].to_string());
                }
                false => {
                    let out = Command::new("bash")
                        .arg("-c")
                        .arg(format!(
                            "pacman -Sl {} | grep {}",
                            pkg.split('/').collect::<Vec<&str>>()[0],
                            pkg.split('/').collect::<Vec<&str>>()[1]
                        ))
                        .stdout(Stdio::null())
                        .status()
                        .expect("Something has gone wrong.");
                    match out.code() {
                        Some(0) => repo.push(reg.replace_all(&pkg, "").to_string()),
                        Some(1) => err_unrec(format!(
                            "Package {} not found in repository {}",
                            pkg.split('/').collect::<Vec<&str>>()[1],
                            pkg.split('/').collect::<Vec<&str>>()[0]
                        )),
                        Some(_) => err_unrec("Something has gone terribly wrong".to_string()),
                        None => err_unrec("Process terminated".to_string()),
                    }
                }
            },
            false => {
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
                            Some(_) => err_unrec("Something has gone terribly wrong".to_string()),
                            None => err_unrec("Process terminated".to_string()),
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
                            Some(_) => err_unrec("Something has gone terribly wrong".to_string()),
                            None => err_unrec("Process terminated".to_string()),
                        }
                    }
                }
            }
        }
    }
    if !as_dep {
        if !repo.is_empty() {
            sec(format!("Installing repo packages: {}", &repo.join(", ")));
            install(noconfirm, false, &repo.join(" "));
        }

        for a in aur {
            sec(format!("Couldn't find {} in repos. Searching AUR", a));
            let md = &rpcinfo(&a).make_depends;
            inssort(noconfirm, true, md.to_vec());
            clone(noconfirm, false, &a);
        }
    } else {
        if !repo.is_empty() {
            sec(format!("Installing repo packages: {}", &repo.join(", ")));
            install(noconfirm, true, &repo.join(" "));
        }

        for a in aur {
            sec(format!("Couldn't find {} in repos. Searching AUR", a));
            let md = &rpcinfo(&a).make_depends;
            inssort(noconfirm, true, md.to_vec());
            clone(noconfirm, true, &a);
        }
    }
}
