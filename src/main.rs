
mod mods;
use mods::{clearcache::clearcache, clone::clone, help::help, install::install, inssort::inssort, search::{a_search, r_search}, uninstall::uninstall, upgrade::upgrade, update::update, ver::ver, strs::inf, strs::err_unrec, strs::err_rec};
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
    if oper == "-S" || oper == "-Sn" || oper == "ins" {
        let pkgs = env::args().skip(2).collect();
        if oper == "-Sn" {
            inssort(true, pkgs);
        } else {
            inssort(false, pkgs);
        }

    // remove
    } else if oper == "-R" || oper == "-Rn " || oper == "-Rsn" || oper == "-Rs" || oper == "rm" {
        if oper == "-Rn" || oper == "-Rsn" {
            for arg in env::args().skip(2) {
                uninstall(true, &arg);
            }
        } else {
            for arg in env::args().skip(2) {
                uninstall(false, &arg);
            }
        }

    // upgrade
    } else if oper == "-Syu" || oper == "-Syun" || oper == "upg" {
        inf(format!("Performing system upgrade"));
        if oper == ("-Syun") {
            upgrade(true, &cache_path);
        } else {
            upgrade(false, &cache_path);
        }

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
    } else if oper == "-h" || oper == "help" {
        help();

    // pacman passthrough
    } else {
        let pass = Command::new("pacman")
            .args(env::args().skip(1))
            .status()
            .expect("Something has gone wrong.");

        match pass.code() {
        Some(1) => {
            err_rec(format!("No such operation \"{}\"", args.join(" ")));
            inf(format!("Try running \"ame help\" for an overview of how to use ame"))
        }
        Some(_) => {}
        None => {
            err_unrec(format!("Something has gone terribly wrong."))
        }}
    }
}
