use crate::{
    err_unrec, inf, inssort, mods::database::add_pkg, mods::strs::prompt, mods::strs::sec,
    mods::strs::succ, mods::purge::purge,
};
use moins::Moins;
use std::{env, fs, path::Path, process::Command};

fn uninstall_make_depend(pkg: &str) { // uninstall make depends of a package
    let make_depends = raur::info(&[&pkg]).unwrap()[0].make_depends.clone();

    let explicit_packages = Command::new("pacman")
        .arg("-Qetq")
        .stdout(std::process::Stdio::piped())
        .output()
        .expect("Something has gone terribly wrong");
    
    
    let expl_pkgs_parse = String::from_utf8(explicit_packages.stdout).unwrap();
    let expl_pkgs_parse = expl_pkgs_parse.split("\n").collect::<Vec<&str>>();

    let mut rem_pkgs = Vec::new();
    for pkg in expl_pkgs_parse {
        for i in 0 .. make_depends.len() {
            match make_depends[i].contains(pkg) {
                false => {
                    match rem_pkgs.contains(&make_depends[i]) {
                        false => {
                            rem_pkgs.push(make_depends[i].as_str().to_string());
                        }
                        _ => {}
                    }
                }
                _ => {}
            };
        }
    }

    if rem_pkgs.len() != 0 {
        inf(format!(
            "{} installed following make dependencies: {}",
            pkg,
            rem_pkgs.join(", ")
        ));
        let remove = prompt(format!("Would you like to remove them?"));
        if remove == true {
            purge(true, rem_pkgs);
        }
    }
    succ(format!("Succesfully installed {}", pkg));
}

pub fn clone(noconfirm: bool, as_dep: bool, pkg: &str) { // clone a package from aur
    let cachedir = format!("{}/.cache/ame", env::var("HOME").unwrap());
    let path = Path::new(&cachedir);
    let pkgdir = format!("{}/{}", &cachedir, &pkg);
    let package = raur::info(&[pkg]).unwrap();

    if package.len() == 0 {
        err_unrec(format!("No matching AUR packages found"));
    }

    let url = format!("https://aur.archlinux.org/{}.git", pkg);

    if !path.is_dir() {
        let cache_result = fs::create_dir(&path);
        match cache_result {
            Ok(_) => inf(format!("Created cache path (first run)")),
            Err(_) => err_unrec(format!("Could not create cache path")),
        }
    }

    inf(format!("Cloning {} ...", pkg));

    if Path::new(&pkgdir).is_dir() {
        let rm_result = fs::remove_dir_all(&pkgdir);
        match rm_result {
            Ok(_) => inf(format!(
                "Package path for {} already found. Removing to reinstall",
                pkg
            )),
            Err(_) => err_unrec(format!(
                "Package path for {} already found, but could not remove to reinstall",
                pkg
            )),
        }
    }

    let dir_result = fs::create_dir(&pkgdir);
    match dir_result {
        Ok(_) => inf(format!("Created package directory for {}", pkg)),
        Err(_) => err_unrec(format!("Couldn't create package directory for {}", pkg)),
    }

    let cd_result = env::set_current_dir(&pkgdir);
    match cd_result {
        Ok(_) => inf(format!("Entered package directory")),
        Err(_) => err_unrec(format!("Could not enter package directory")),
    }

    sec(format!("Installing AUR package depends"));

    inssort(noconfirm, true, package[0].depends.clone());

    let clone = std::process::Command::new("git")
        .arg("clone")
        .arg(&url)
        .arg(&pkgdir)
        .status()
        .expect("couldnt clone repository");
    match clone.code() {
        Some(0) => {
            inf(format!("Cloning {} into package directory", pkg));
        }
        Some(_) => err_unrec(format!("Failed cloning {} into package directory", pkg)),
        _ => err_unrec(format!("Failed cloning {} into package directory", pkg)),
    }
    if as_dep == false {
        if noconfirm == false {
            let pkgbuild = prompt(format!("View PKGBUILD?"));

            if pkgbuild == true {
                let mut pkgbld = fs::read_to_string(format!("{}/PKGBUILD", &pkgdir)).unwrap();
                Moins::run(&mut pkgbld, None);
            }
        }

        if noconfirm == true {
            sec(format!("Installing {} ...", pkg));
            let install_result = Command::new("makepkg")
                .arg("-si")
                .arg("--noconfirm")
                .arg("--needed")
                .status();
            match install_result {
                Ok(_) => {
                    uninstall_make_depend(pkg);
                    add_pkg(false, pkg);
                }
                Err(_) => {
                    err_unrec(format!("Couldn't install {}", pkg));
                }
            };
        } else {
            sec(format!("Installing {} ...", pkg));
            let install_result = Command::new("makepkg")
                .arg("-si")
                .arg("--needed")
                .status()
                .expect("Couldn't call makepkg");
            match install_result.code() {
                Some(0) => {
                    uninstall_make_depend(pkg);
                    add_pkg(false, pkg);
                }
                Some(_) => {
                    err_unrec(format!("Couldn't install {}", pkg));
                }
                None => {
                    err_unrec(format!("Couldn't install {}", pkg));
                }
            };
        }
    } else {
        sec(format!("Installing {} ...", pkg));
            let install_result = Command::new("makepkg")
                .arg("-si")
                .arg("--noconfirm")
                .arg("--needed")
                .arg("--asdeps")
                .status();
            match install_result {
                Ok(_) => {
                    uninstall_make_depend(pkg);
                    add_pkg(false, pkg);
                }
                Err(_) => {
                    err_unrec(format!("Couldn't install {}", pkg));
                }
            };
    }
}
