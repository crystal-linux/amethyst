mod mods;
use mods::{clearcache::clearcache, clone::clone, help::help, install::install, search::{a_search, r_search}, uninstall::uninstall, upgrade::upgrade, ver::ver};
use std::{env, process::exit, process::Command};

fn main() {
    // let statements
    let args: Vec<String> = env::args().collect();
    let homepath = std::env::var("HOME").unwrap();
    let cache_path = format!("/{}/.cache/ame/", homepath);
    
    // args catch
    if args.len() <= 1 {
        help();
        exit(1);
    }

    let oper = &args[1];

    // install
    if oper == "-S" || oper == "ins" || oper == "install" {
        for arg in env::args().skip(2) {
            let out = Command::new("pacman")
                              .arg("-Ss")
                              .arg(&arg)
                              .arg(" > /dev/null && return ${PIPESTATUS}")
                              .status()
                              .unwrap();
            if out.success() {
                install(&arg);
            } else {
                clone(&arg);
            }
        }

    // remove
    } else if oper == "-R" || oper == "-Rs" || oper=="rem" || oper=="remove" {
        for arg in env::args().skip(2) {
            uninstall(&arg);
        }

    // upgrade
    } else if oper == "-Syu" || oper=="upg" || oper=="upgrade" {
        upgrade(&cache_path);
    } else if oper == "-Ss" || oper=="sea" || oper=="search" {
        for arg in env::args().skip(2) {
            r_search(&arg);
            a_search(&arg);
        }

    // aur search
    } else if oper == "-Sa" || oper=="aursea" || oper=="aursearch" {
        for arg in env::args().skip(2) {
            a_search(&arg);
        }

    // repo search
    } else if oper == "-Sr" || oper=="repsea" || oper=="reposearch" {
        for arg in env::args().skip(2) {
            r_search(&arg);
        }

    // clear cache
    } else if oper == "-Cc" || oper=="clr" || oper=="clear-cache" {
        clearcache();

    // version / contrib
    } else if oper == "-v" || oper == "-V" || oper == "ver" {
        ver();

    // help
    } else {
        help();
        exit(0);
    }
}
