mod mods;
use mods::{clearcache::clearcache, clone::clone, help::help, install::install, search::{a_search, r_search}, uninstall::uninstall, upgrade::upgrade, update::update, ver::ver, strs::inf};
use std::{env, process::exit, process::Command, process::Stdio};

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
    if oper == "-S" || oper == "ins" {
        for arg in env::args().skip(2) {
            let out = Command::new("pacman")
                              .arg("-Ss")
                              .arg(&arg)
                              .stdout(Stdio::null())
                              .status()
                              .expect("");
            if out.code() == Some(0) {
                inf(format!("Installing {}", arg));
                install(&arg);
            } else {
                inf(format!("Cloning {} from the AUR", arg));
                clone(&arg);
            }
        }

    // remove
    } else if oper == "-R" || oper == "-Rs" || oper == "rm" {
        for arg in env::args().skip(2) {
            uninstall(&arg);
        }

    // upgrade
    } else if oper == "-Syu" || oper == "upg" {
        upgrade(&cache_path);

    // update
    } else if oper == "-Sy" || oper == "upd" {
        update();

    // general search
    } else if oper == "-Ss" || oper == "sea" {
        for arg in env::args().skip(2) {
            r_search(&arg);
            a_search(&arg);
        }

    // aur search
    } else if oper == "-Sa" || oper == "aursea" {
        for arg in env::args().skip(2) {
            a_search(&arg);
        }

    // repo search
    } else if oper == "-Sr" || oper == "repsea" {
        for arg in env::args().skip(2) {
            r_search(&arg);
        }

    // clear cache !! DEBUG ONLY !! DO NOT DO THIS IF YOU DONT KNOW WHAT YOURE DOING !!
    } else if oper == "-Cc" || oper == "clr" {
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
