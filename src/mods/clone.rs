use crate::{
    err_unrec, inf, mods::database::add_pkg, mods::purge::purge, mods::rpc::*, mods::strs::prompt,
    mods::strs::sec, mods::strs::succ,
};
use moins::Moins;
use std::{env, fs, path::Path, process::Command};

fn uninstall_make_depend(pkg: &str) {
    // uninstall make depends of a package

    // gets the "make_depends" field of the package using rpcinfo()
    // let make_depends = rpcinfo(pkg)[0].make_depends;
    let make_depends = rpcinfo(pkg).make_depends;

    let explicit_packages = Command::new("pacman")
        .arg("-Qetq")
        .stdout(std::process::Stdio::piped())
        .output()
        .expect("Something has gone terribly wrong");

    let expl_pkgs_parse = String::from_utf8(explicit_packages.stdout).unwrap();
    let expl_pkgs_parse = expl_pkgs_parse.split('\n').collect::<Vec<&str>>();

    let mut rem_pkgs: Vec<String> = Vec::new();
    for pkg in expl_pkgs_parse {
        for md in &make_depends {
            if let false = md.contains(pkg) {
                if let false = rem_pkgs.contains(md) {
                    rem_pkgs.push(md.as_str().to_string());
                }
            };
        }
    }

    if !rem_pkgs.is_empty() {
        inf(format!(
            "{} installed following make dependencies: {}",
            pkg,
            rem_pkgs.join(", ")
        ));
        let remove = prompt("Would you like to remove them?".to_string());
        if remove {
            purge(true, rem_pkgs);
        }
    }
    succ(format!("Succesfully installed {}", pkg));
}

pub fn clone(noconfirm: bool, as_dep: bool, pkg: &str) {
    // clone a package from aur
    let cachedir = format!("{}/.cache/ame", env::var("HOME").unwrap());
    let path = Path::new(&cachedir);
    let pkgdir = format!("{}/{}", &cachedir, &pkg);
    let search = rpcsearch(pkg).results;
    if search.is_empty() {
        err_unrec("No matching AUR packages found".to_string());
    }

    let url = format!("https://aur.archlinux.org/{}.git", pkg);

    if !Path::new(&format!("{}/.cache", env::var("HOME").unwrap())).exists() {
        fs::create_dir_all(format!("{}/.cache", env::var("HOME").unwrap()))
            .expect("Failed to create ~/.cache directory");
    }
    if !path.is_dir() {
        let cache_result = fs::create_dir(&path);
        match cache_result {
            Ok(_) => inf("Created cache path (first run)".to_string()),
            Err(_) => err_unrec("Could not create cache path".to_string()),
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
        Ok(_) => inf("Entered package directory".to_string()),
        Err(_) => err_unrec("Could not enter package directory".to_string()),
    }

    sec("Installing AUR package depends".to_string());

    // inssort(noconfirm, true, package[0].depends.clone());

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
    if !as_dep {
        if !noconfirm {
            let pkgbuild = prompt("View PKGBUILD?".to_string());

            if pkgbuild {
                let mut pkgbld = fs::read_to_string(format!("{}/PKGBUILD", &pkgdir)).unwrap();
                Moins::run(&mut pkgbld, None);
            }
        }

        sec(format!("Installing {} ...", pkg));
        if noconfirm {
            let install_result = Command::new("makepkg")
                .arg("-si")
                .arg("--noconfirm")
                .arg("--needed")
                .status();
            match install_result {
                Ok(_) => {
                    uninstall_make_depend(pkg);
                    let vec = vec![pkg];
                    add_pkg(false, &vec);
                }
                Err(_) => {
                    err_unrec(format!("Couldn't install {}", pkg));
                }
            };
        } else {
            let install_result = Command::new("makepkg")
                .arg("-si")
                .arg("--needed")
                .status()
                .expect("Couldn't call makepkg");
            match install_result.code() {
                Some(0) => {
                    uninstall_make_depend(pkg);
                    let vec = vec![pkg];
                    add_pkg(false, &vec);
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
                let vec = vec![pkg];
                add_pkg(false, &vec);
            }
            Err(_) => {
                err_unrec(format!("Couldn't install {}", pkg));
            }
        };
    }
}
