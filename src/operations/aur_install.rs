use std::env::set_current_dir;
use std::fs::remove_dir_all;
use std::path::Path;
use std::process::Command;
use std::{env, fs};

use crate::internal::commands::ShellCommand;
use crate::internal::error::SilentUnwrap;
use crate::internal::exit_code::AppExitCode;
use crate::internal::rpc::rpcinfo;
use crate::{crash, info, log, prompt, Options};

pub fn aur_install(a: Vec<String>, options: Options) {
    // Initialise variables
    let url = crate::internal::rpc::URL;
    let cachedir = format!("{}/.cache/ame/", env::var("HOME").unwrap());
    let verbosity = options.verbosity;
    let noconfirm = options.noconfirm;

    if verbosity >= 1 {
        log!("Installing from AUR: {:?}", &a);
    }

    info!("Installing packages {} from the AUR", a.join(", "));

    for package in a {
        // Query AUR for package info
        let rpcres = rpcinfo(package);

        if !rpcres.found {
            // If package isn't found, break
            break;
        }

        // Get package name
        let pkg = &rpcres.package.as_ref().unwrap().name;

        if verbosity >= 1 {
            log!("Cloning {} into cachedir", pkg);
        }

        info!("Cloning package source");

        // Clone package into cachedir
        set_current_dir(Path::new(&cachedir)).unwrap();
        ShellCommand::git()
            .arg("clone")
            .arg(format!("{}/{}", url, pkg))
            .wait()
            .silent_unwrap(AppExitCode::GitError);

        if verbosity >= 1 {
            log!(
                "Cloned {} into cachedir, moving on to resolving dependencies",
                pkg
            );
            log!(
                "Raw dependencies for package {} are:\n{:?}",
                pkg,
                rpcres.package.as_ref().unwrap().depends.join(", ")
            );
            log!(
                "Raw makedepends for package {} are:\n{:?}",
                pkg,
                rpcres.package.as_ref().unwrap().make_depends.join(", ")
            );
        }

        // Sort dependencies and makedepends
        log!("Sorting dependencies");
        let sorted = crate::internal::sort(&rpcres.package.as_ref().unwrap().depends, options);
        log!("Sorting make dependencies");
        let md_sorted =
            crate::internal::sort(&rpcres.package.as_ref().unwrap().make_depends, options);

        if verbosity >= 1 {
            log!("Sorted dependencies for {} are:\n{:?}", pkg, &sorted);
            log!("Sorted makedepends for {} are:\n{:?}", pkg, &md_sorted);
        }

        // Create newopts struct for installing dependencies
        let newopts = Options {
            verbosity,
            noconfirm,
            asdeps: true,
        };

        // If dependencies are not found in AUR or repos, crash
        if !sorted.nf.is_empty() || !md_sorted.nf.is_empty() {
            crash!(
                AppExitCode::MissingDeps,
                "Could not find dependencies {} for package {}, aborting",
                sorted.nf.join(", "),
                pkg,
            );
        }

        if !noconfirm {
            // Prompt user to view PKGBUILD
            let p1 = prompt!(default false,
                "Would you like to review {}'s PKGBUILD (and any .install files if present)?",
                pkg
            );
            let editor: &str = &env::var("PAGER").unwrap_or_else(|_| "less".parse().unwrap());

            if p1 {
                // Open PKGBUILD in pager
                Command::new(editor)
                    .arg(format!("{}/PKGBUILD", pkg))
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();

                // Check if any .install files are present
                let status = ShellCommand::bash()
                    .arg("-c")
                    .arg(format!("ls {}/*.install &> /dev/null", pkg))
                    .wait()
                    .silent_unwrap(AppExitCode::Other);

                if status.success() {
                    // If so, open them too
                    ShellCommand::bash()
                        .arg("-c")
                        .arg(format!("{} {}/*.install", editor, pkg))
                        .wait()
                        .silent_unwrap(AppExitCode::Other);
                }

                // Prompt user to continue
                let p2 = prompt!(default true, "Would you still like to install {}?", pkg);
                if !p2 {
                    // If not, crash
                    fs::remove_dir_all(format!("{}/{}", cachedir, pkg)).unwrap();
                    crash!(AppExitCode::UserCancellation, "Not proceeding");
                }
            }
        }

        info!("Moving on to install dependencies");

        // Install dependencies and makedepends
        if !sorted.repo.is_empty() {
            crate::operations::install(sorted.repo, newopts);
            crate::operations::install(md_sorted.repo, newopts);
        }
        if !sorted.aur.is_empty() {
            crate::operations::aur_install(sorted.aur, newopts);
            crate::operations::aur_install(md_sorted.aur, newopts);
        }

        // Build makepkg args
        let mut makepkg_args = vec!["-rsci", "--skippgp"];
        if options.asdeps {
            makepkg_args.push("--asdeps")
        }
        if options.noconfirm {
            makepkg_args.push("--noconfirm")
        }

        info!("Building time!");

        // Enter cachedir and build package
        set_current_dir(format!("{}/{}", cachedir, pkg)).unwrap();
        let status = ShellCommand::makepkg()
            .args(makepkg_args)
            .wait()
            .silent_unwrap(AppExitCode::MakePkgError);

        if !status.success() {
            // If build failed, crash
            fs::remove_dir_all(format!("{}/{}", cachedir, pkg)).unwrap();
            crash!(
                AppExitCode::PacmanError,
                "Error encountered while installing {}, aborting",
                pkg,
            );
        }

        // Return to cachedir
        set_current_dir(&cachedir).unwrap();

        // Remove package from cache
        remove_dir_all(format!("{}/{}", cachedir, &pkg)).unwrap();
    }
}
