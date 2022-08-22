use std::env::set_current_dir;
use std::path::Path;
use std::process::Command;
use std::{env, fs};

use crate::internal::commands::ShellCommand;
use crate::internal::config;
use crate::internal::error::SilentUnwrap;
use crate::internal::exit_code::AppExitCode;
use crate::internal::rpc::rpcinfo;
use crate::{crash, info, log, prompt, warn, Options};

fn list(dir: &str) -> Vec<String> {
    let dirs = fs::read_dir(Path::new(&dir)).unwrap();
    let dirs: Vec<String> = dirs
        .map(|dir| {
            (*dir
                .unwrap()
                .path()
                .to_str()
                .unwrap()
                .split('/')
                .collect::<Vec<&str>>()
                .last()
                .unwrap())
            .to_string()
        })
        .collect();
    dirs
}

fn mktemp() -> String {
    let tempdir = Command::new("mktemp")
        .args(&["-d", "/tmp/ame.XXXXXX.tmp"])
        .output()
        .unwrap()
        .stdout;

    String::from_utf8(tempdir).unwrap().trim().to_string()
}

fn review(cachedir: &str, pkg: &str, orig_cachedir: &str) {
    // Prompt user to view PKGBUILD
    let p0 = prompt!(default false, "Would you like to review and/or edit {}'s PKGBUILD (and any adjacent build files if present)?", pkg);
    if p0 {
        info!("This will drop you into a standard `bash` shell (unless set otherwise in the config) in the package's cache directory. If any changes are made, you will be prompted whether to save them to your home directory. To stop reviewing/editing, just run `exit`");
        let p1 = prompt!(default true,
            "Continue?"
        );

        if p1 {
            let config = config::read();
            let cdir = env::current_dir().unwrap().to_str().unwrap().to_string();
            set_current_dir(Path::new(&format!("{}/{}", &cachedir, pkg))).unwrap();

            if config.extra.review_user_shell {
                Command::new(&env::var("SHELL").unwrap())
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
            } else {
                ShellCommand::bash().wait().unwrap();
            }

            set_current_dir(Path::new(&cdir)).unwrap();

            // Prompt user to save changes
            let p2 = prompt!(default false,
                "Save changes to package {}?",
                pkg
            );
            if p2 {
                // Save changes to ~/.local/share
                let dest = format!(
                    "{}-saved-{}",
                    pkg,
                    chrono::Local::now()
                        .naive_local()
                        .format("%Y-%m-%d_%H-%M-%S")
                );
                Command::new("cp")
                    .arg("-r")
                    .arg(format!("{}/{}", cachedir, pkg))
                    .arg(format!(
                        "{}/.local/share/ame/{}",
                        env::var("HOME").unwrap(),
                        dest
                    ))
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();

                // Alert user
                info!("Saved changes to ~/.local/share/ame/{}", dest);
            };
        }
    }

    // Prompt user to continue
    let p = prompt!(default true, "Would you still like to install {}?", pkg);
    if !p {
        // If not, crash
        if orig_cachedir.is_empty() {
            fs::remove_dir_all(format!("{}/{}", cachedir, pkg)).unwrap();
        }
        crash!(AppExitCode::UserCancellation, "Not proceeding");
    };
}

fn finish(cachedir: &str, pkg: &str, options: &Options) {
    // Install all packages from cachedir except `pkg` using --asdeps
    let dirs = list(cachedir);

    // Get a list of packages in cachedir
    if dirs.len() > 1 {
        info!("Installing all AUR dependencies");
        let cmd = std::process::Command::new("bash")
            .args(&[
                "-cO",
                "extglob",
                format!("sudo pacman -U --asdeps {}/!({})/*.zst {}", cachedir, pkg, if options.noconfirm { "--noconfirm" } else { "" }).as_str(),
            ])
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
        if cmd.success() {
            info!("All AUR dependencies installed");
        } else {
            crash!(AppExitCode::PacmanError, "AUR dependencies failed to install");
        }
    }

    // Install package explicitly
    info!("Installing {}", pkg);
    let cmd = std::process::Command::new("bash")
        .args(&[
            "-c",
            format!("sudo pacman -U {}/{}/*.zst {}", cachedir, pkg, if options.noconfirm { "--noconfirm" } else { "" }).as_str(),
        ])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    if cmd.success() {
        info!("{} installed!", pkg);
    } else {
        crash!(AppExitCode::PacmanError, "{} failed to install", pkg);
    }
}

pub fn aur_install(a: Vec<String>, options: Options, orig_cachedir: &str) {
    // Initialise variables
    let url = crate::internal::rpc::URL;
    let cachedir = if options.asdeps || !orig_cachedir.is_empty() {
        orig_cachedir.to_string()
    } else {
        mktemp()
    };
    let verbosity = options.verbosity;
    let noconfirm = options.noconfirm;

    if verbosity >= 1 {
        log!("Installing from AUR: {:?}", &a);
    }

    info!("Installing packages {} from the AUR", a.join(", "));

    let mut failed: Vec<String> = vec![];

    for package in a {
        let dirs = list(&cachedir);
        // Don't process packages if they are already in the cachedir
        if dirs.contains(&package) {
            continue;
        }

        // Query AUR for package info
        let rpcres = rpcinfo(&package);

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
                "Cloned {} into cachedir, moving on to resolving dependencies: {:?}",
                pkg,
                rpcres.package
            );
        }

        // Sort dependencies and makedepends
        if verbosity >= 1 {
            log!("Sorting dependencies and makedepends");
        }
        let mut sorted = crate::internal::sort(&rpcres.package.as_ref().unwrap().depends, options);
        let mut md_sorted =
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

        // Get a list of installed packages
        let installed = ShellCommand::pacman()
            .elevated()
            .args(&["-Qq"])
            .wait_with_output()
            .silent_unwrap(AppExitCode::PacmanError)
            .stdout
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| (*s).to_string())
            .collect::<Vec<String>>();

        // Remove installed packages from sorted dependencies and makedepends
        if verbosity >= 1 {
            log!("Removing installed packages from sorted dependencies and makedepends");
        }
        sorted.aur.retain(|x| !installed.contains(x));
        sorted.repo.retain(|x| !installed.contains(x));

        md_sorted.aur.retain(|x| !installed.contains(x));
        md_sorted.repo.retain(|x| !installed.contains(x));

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
            review(&cachedir, pkg, orig_cachedir);
        }

        info!("Moving on to install dependencies");

        // Install dependencies and makedepends
        if !sorted.repo.is_empty() {
            crate::operations::install(&sorted.repo, newopts);
        }
        if !sorted.aur.is_empty() {
            crate::operations::aur_install(sorted.aur, newopts, &cachedir.clone());
        }
        if !md_sorted.repo.is_empty() {
            crate::operations::install(&md_sorted.repo, newopts);
        }
        if !md_sorted.aur.is_empty() {
            crate::operations::aur_install(md_sorted.aur, newopts, &cachedir.clone());
        }

        // Build makepkg args
        let mut makepkg_args = vec!["-rcd", "--skippgp", "--needed"];
        if options.asdeps {
            makepkg_args.push("--asdeps");
        }
        if options.noconfirm {
            makepkg_args.push("--noconfirm");
        }

        info!("Building time!");

        // Enter cachedir and build package
        set_current_dir(format!("{}/{}", cachedir, pkg)).unwrap();

        let status = ShellCommand::makepkg()
            .args(makepkg_args)
            .wait()
            .silent_unwrap(AppExitCode::MakePkgError);

        if !status.success() {
            // If build failed, push to failed vec
            failed.push(pkg.clone());
            return;
        }

        // Return to cachedir
        set_current_dir(&cachedir).unwrap();

        if !options.asdeps {
            finish(&cachedir, pkg, &options);
        }
    }

    // If any packages failed to build, warn user with failed packages
    if !failed.is_empty() {
        let failed_str = format!("{}.failed", cachedir);
        warn!(
            "Failed to build packages {}, keeping cache directory at {} for manual inspection",
            failed.join(", "),
            if orig_cachedir.is_empty() {
                &cachedir
            } else {
                &failed_str
            }
        );
        if orig_cachedir.is_empty() {
            Command::new("mv")
                .args(&[&cachedir, &format!("{}.failed", cachedir)])
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
        }
    } else if !options.asdeps && orig_cachedir.is_empty() {
        rm_rf::remove(&cachedir).unwrap_or_else(|e|
            crash!(AppExitCode::Other, "Could not remove cache directory at {}: {}. This could be a permissions issue with fakeroot, try running `sudo rm -rf {}`", cachedir, e, cachedir)
        );
    }
}
