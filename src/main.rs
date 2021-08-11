mod mods;
use mods::{clearcache::clearcache, clone::clone, help::help, install::install, search::{a_search, r_search}, uninstall::uninstall, upgrade::upgrade, flatpak::flatpak};
use std::{env, process::exit, process::Command};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        help();
        exit(1);
    }

    let oper = &args[1];
    if oper == "-S" {
        for arg in env::args().skip(2) {
            let out = Command::new("pacman").arg("-Ss").arg(&arg).status().unwrap();
            if out.success() {
                install(&arg);
            } else {
                clone(&arg);
            }
        }
    } else if oper == "-R" {
        for arg in env::args().skip(2) {
            uninstall(&arg);
        }
    } else if oper == "-Syu" {
        upgrade();
    } else if oper == "-Ss" {
        for arg in env::args().skip(2) {
            r_search(&arg);
            a_search(&arg);
        }
    } else if oper == "-Sa" {
        for arg in env::args().skip(2) {
            a_search(&arg);
        }
    } else if oper == "-Sr" {
        for arg in env::args().skip(2) {
            r_search(&arg);
        }
    } else if oper == "-Cc" {
        clearcache();
    } else if oper == "-f" {
        for arg in env::args().skip(2) {
            flatpak(&arg);
        }
    } else {
        help();
        exit(0);
    }
}
