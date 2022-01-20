use std::env;
use std::env::set_current_dir;
use std::fs::remove_dir_all;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::internal::rpc::rpcinfo;
use crate::internal::{crash, prompt};
use crate::{info, log, Options};

pub fn aur_install(a: Vec<String>, options: Options) {
    let url = crate::internal::rpc::URL;
    let cachedir = format!("{}/.cache/ame/", env::var("HOME").unwrap());
    let verbosity = options.verbosity;
    let noconfirm = options.noconfirm;

    if verbosity >= 1 {
        log(format!("Installing from AUR: {:?}", &a));
    }

    info(format!("Installing packages {} from the AUR", a.join(", ")));

    for package in a {
        let rpcres = rpcinfo(package);

        if !rpcres.found {
            break;
        }

        let pkg = &rpcres.package.as_ref().unwrap().name;

        if verbosity >= 1 {
            log(format!("Cloning {} into cachedir", pkg));
        }

        info("Cloning package source".to_string());

        set_current_dir(Path::new(&cachedir)).unwrap();
        Command::new("git")
            .arg("clone")
            .arg(format!("{}/{}", url, pkg))
            .stdout(Stdio::null())
            .status()
            .expect("Something has gone wrong");

        if verbosity >= 1 {
            log(format!(
                "Cloned {} into cachedir, moving on to resolving dependencies",
                pkg
            ));
            log(format!(
                "Raw dependencies for package {} are:\n{:?}",
                pkg,
                rpcres.package.as_ref().unwrap().depends.join(", ")
            ));
        }

        // dep sorting
        info("Sorting dependencies".to_string());
        let sorted = crate::internal::sort(&rpcres.package.as_ref().unwrap().depends, options);

        if verbosity >= 1 {
            log(format!(
                "Sorted dependencies for {} are:\n{:?}",
                pkg, &sorted
            ));
        }

        let newopts = Options {
            verbosity,
            noconfirm,
            asdeps: true,
        };

        if !sorted.nf.is_empty() {
            crash(
                format!(
                    "Could not find dependencies {} for package {}, aborting",
                    sorted.nf.join(", "),
                    pkg
                ),
                1,
            );
        }

        if !noconfirm {
            let p = prompt(
                "Would you like to view or edit {}'s PKGBUILD?".to_string(),
                false,
            );
            let editor = env::var("EDITOR").unwrap_or_else(|_| "nano".parse().unwrap());

            if p {
                Command::new(editor)
                    .arg(format!("{}/PKGBUILD", pkg))
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
            }
        }

        // dep installing
        info("Moving on to install dependencies".to_string());
        crate::operations::install(sorted.repo, newopts);
        crate::operations::aur_install(sorted.aur, newopts);

        let mut makepkg_args = vec!["-rsic", "--needed"];
        if options.asdeps {
            makepkg_args.push("--asdeps")
        }
        if options.noconfirm {
            makepkg_args.push("--noconfirm")
        }

        // package building and installing
        info("Building time!".to_string());
        set_current_dir(format!("{}/{}", cachedir, pkg)).unwrap();
        let out = Command::new("makepkg")
            .args(&makepkg_args)
            .status()
            .expect("Something has gone wrong");

        if out.code() != Some(0) {
            crash(
                format!("Error encountered while installing {}, aborting", pkg),
                1,
            );
        }

        set_current_dir(&cachedir).unwrap();
        remove_dir_all(format!("{}/{}", cachedir, &pkg)).unwrap();

        // pushes package to database
        crate::database::add(rpcres.package.unwrap(), options);
    }
}
