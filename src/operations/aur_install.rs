use async_recursion::async_recursion;
use std::env;
use std::env::set_current_dir;
use std::path::{Path, PathBuf};
use std::process::Command;
use tokio::fs;

use crate::internal::commands::ShellCommand;
use crate::internal::error::SilentUnwrap;
use crate::internal::exit_code::AppExitCode;
use crate::internal::rpc::rpcinfo;
use crate::{crash, info, internal::fs_utils::rmdir_recursive, log, prompt, Options};

/// Installs a given list of packages from the aur
#[async_recursion]
pub async fn aur_install(packages: Vec<String>, options: Options) {
    let url = crate::internal::rpc::URL;
    let cachedir = format!("{}/.cache/ame/", env::var("HOME").unwrap());
    let verbosity = options.verbosity;
    let noconfirm = options.noconfirm;

    if verbosity >= 1 {
        log!("Installing from AUR: {:?}", &packages);
    }

    info!("Installing packages {} from the AUR", packages.join(", "));

    for package_name in packages {
        let rpcres = rpcinfo(&package_name)
            .await
            .silent_unwrap(AppExitCode::RpcError);

        if rpcres.is_none() {
            break;
        }

        let package = rpcres.unwrap();
        let pkg_name = package.metadata.name;

        if verbosity >= 1 {
            log!("Cloning {} into cachedir", pkg_name);
        }

        info!("Cloning package source");

        set_current_dir(Path::new(&cachedir)).unwrap();
        ShellCommand::git()
            .arg("clone")
            .arg(format!("{}/{}", url, pkg_name))
            .wait()
            .await
            .silent_unwrap(AppExitCode::GitError);

        if verbosity >= 1 {
            log!(
                "Cloned {} into cachedir, moving on to resolving dependencies",
                pkg_name
            );
            log!(
                "Raw dependencies for package {} are:\n{:?}",
                pkg_name,
                package.depends,
            );
            log!(
                "Raw makedepends for package {} are:\n{:?}",
                pkg_name,
                package.make_depends.join(", "),
            );
        }

        // dep sorting
        log!("Sorting dependencies");
        let sorted = crate::internal::sort(&package.depends, options).await;
        log!("Sorting make dependencies");
        let md_sorted = crate::internal::sort(&package.make_depends, options).await;

        if verbosity >= 1 {
            log!("Sorted dependencies for {} are:\n{:?}", pkg_name, &sorted);
            log!("Sorted makedepends for {} are:\n{:?}", pkg_name, &md_sorted);
        }

        let newopts = Options {
            verbosity,
            noconfirm,
            asdeps: true,
        };

        if !sorted.nf.is_empty() || !md_sorted.nf.is_empty() {
            crash!(
                AppExitCode::MissingDeps,
                "Could not find dependencies {} for package {}, aborting",
                sorted.nf.join(", "),
                pkg_name,
            );
        }

        if !noconfirm {
            let p1 = prompt!(default false,
                "Would you like to review {}'s PKGBUILD (and any .install files if present)?",
                pkg_name,
            );
            let editor: &str = &env::var("PAGER").unwrap_or_else(|_| "less".parse().unwrap());

            if p1 {
                Command::new(editor)
                    .arg(format!("{}/PKGBUILD", pkg_name))
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();

                let status = ShellCommand::bash()
                    .arg("-c")
                    .arg(format!("ls {}/*.install &> /dev/null", pkg_name))
                    .wait()
                    .await
                    .silent_unwrap(AppExitCode::Other);

                if status.success() {
                    ShellCommand::bash()
                        .arg("-c")
                        .arg(format!("{} {}/*.install", editor, pkg_name))
                        .wait()
                        .await
                        .silent_unwrap(AppExitCode::Other);
                }

                let p2 = prompt!(default true, "Would you still like to install {}?", pkg_name);
                if !p2 {
                    fs::remove_dir_all(format!("{}/{}", cachedir, pkg_name))
                        .await
                        .unwrap();
                    crash!(AppExitCode::UserCancellation, "Not proceeding");
                }
            }
        }

        // dep installing
        info!("Moving on to install dependencies");

        if !sorted.repo.is_empty() {
            crate::operations::install(sorted.repo, newopts).await;
            crate::operations::install(md_sorted.repo, newopts).await;
        }
        if !sorted.aur.is_empty() {
            crate::operations::aur_install(sorted.aur, newopts).await;
            crate::operations::aur_install(md_sorted.aur, newopts).await;
        }

        let mut makepkg_args = vec!["-rsci", "--skippgp"];
        if options.asdeps {
            makepkg_args.push("--asdeps")
        }
        if options.noconfirm {
            makepkg_args.push("--noconfirm")
        }

        // package building and installing
        info!("Building time!");
        set_current_dir(format!("{}/{}", cachedir, pkg_name)).unwrap();
        let status = ShellCommand::makepkg()
            .args(makepkg_args)
            .wait()
            .await
            .silent_unwrap(AppExitCode::MakePkgError);

        if !status.success() {
            fs::remove_dir_all(format!("{}/{}", cachedir, pkg_name))
                .await
                .unwrap();
            crash!(
                AppExitCode::PacmanError,
                "Error encountered while installing {}, aborting",
                pkg_name,
            );
        }

        set_current_dir(&cachedir).unwrap();
        let package_cache = PathBuf::from(format!("{cachedir}/{pkg_name}"));
        rmdir_recursive(&package_cache).await.unwrap()
    }
}
