use crate::{
    err_unrec, inf, inssort, mods::database::add_pkg, mods::strs::prompt, mods::strs::sec,
    mods::strs::succ, mods::uninstall::uninstall,
};
use git2::Repository;
use moins::Moins;
use std::{env, fs, path::Path, process::Command};

fn uninstall_make_depend(pkg: &str) {
    let make_depends = raur::info(&[&pkg]).unwrap()[0].make_depends.clone();

    if make_depends.len() != 0 {
        inf(format!(
            "{} installed following make dependencies: {}",
            pkg,
            make_depends.join(", ")
        ));
        let remove = prompt(format!("Would you like to remove them?"));
        if remove == true {
            uninstall(true, make_depends);
        }
    }
    succ(format!("Succesfully installed {}", pkg));
}

pub fn clone(noconfirm: bool, pkg: &str) {
    let cachedir = format!("{}/.cache/ame", std::env::var("HOME").unwrap());
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

    inssort(noconfirm, package[0].depends.clone());

    let clone = Repository::clone(&url, Path::new(&pkgdir));
    match clone {
        Ok(_) => {
            inf(format!("Cloning {} into package directory", pkg));
        }
        Err(_) => err_unrec(format!("Failed cloning {} into package directory", pkg)),
    }

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
                let add_pkg_res = add_pkg(false, pkg);
                match add_pkg_res {
                    Ok(_) => inf(format!("Added package {} to database", pkg)),
                    Err(_) => err_unrec(format!("Couldn't add package {} to database", pkg)),
                }
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
                let add_pkg_res = add_pkg(false, pkg);
                match add_pkg_res {
                    Ok(_) => inf(format!("Added package {} to database", pkg)),
                    Err(_) => err_unrec(format!("Couldn't add package {} to database", pkg)),
                }
            }
            Some(_) => {
                err_unrec(format!("Couldn't install {}", pkg));
            }
            None => {
                err_unrec(format!("Couldn't install {}", pkg));
            }
        };
    }
}
