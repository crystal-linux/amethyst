
mod mods;
use mods::{clearcache::clearcache, clone::clone, help::help, install::install, inssort::inssort, search::{a_search, r_search}, uninstall::uninstall, upgrade::upgrade, update::update, ver::ver, strs::inf, strs::err_unrec, strs::err_rec, xargs::*, database::addPkg};
use std::{env, process::exit, process::Command};

fn main() {
    // let statements
    let args: Vec<String> = env::args().collect();
    let mut pkgs: Vec<String> = env::args().skip(2).collect();
    
    // args catch
    if args.len() <= 1 {
        help();
        exit(1);
    }

    let oper = &args[1];
    let noconfirm: bool = noconf(&args);

    argssort(&mut pkgs);

    // install
    if oper == "-S" || oper == "-Sn" || oper == "ins" {
        inssort(noconfirm, pkgs);

    // remove
    } else if oper == "-R" || oper == "-Rn " || oper == "-Rsn" || oper == "-Rs" || oper == "rm" {
        uninstall(noconfirm, pkgs);

    // upgrade
    } else if oper == "-Syu" || oper == "-Syun" || oper == "upg" {
            upgrade(noconfirm);

    // update
    } else if oper == "-Sy" || oper == "upd" {
        update();

    // general search
    } else if oper == "-Ss" || oper == "sea" {
        r_search(&args[2]);
        a_search(&args[2]);

    // aur search
    } else if oper == "-Sa" || oper == "aursea" {
        a_search(&args[2]);

    // repo search
    } else if oper == "-Sr" || oper == "repsea" {
        r_search(&args[2]);

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
