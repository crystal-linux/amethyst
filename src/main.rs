mod mods;
use mods::{
    clearcache::clearcache,
    clone::clone,
    help::help,
    inssort::inssort,
    install::install,
    search::{a_search, r_search},
    strs::err_rec,
    strs::err_unrec,
    strs::inf,
    uninstall::uninstall,
    update::update,
    upgrade::upgrade,
    ver::ver,
    xargs::*,
    database::{addPkg, remPkg}
};
use std::{env, process::exit, process::Command};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut pkgs: Vec<String> = env::args().skip(2).collect();

    if args.len() <= 1 {
        help();
        exit(1);
    }

    let oper = &args[1];
    let noconfirm: bool = noconf(&args);

    argssort(&mut pkgs);

    // at some point weve GOT TO rework this into a `match` statement

    if oper == "-S" || oper == "-Sn" || oper == "ins" {
        inssort(noconfirm, pkgs);
    } else if oper == "-R" || oper == "-Rn " || oper == "-Rsn" || oper == "-Rs" || oper == "rm" {
        uninstall(noconfirm, pkgs);
    } else if oper == "-Syu" || oper == "-Syun" || oper == "upg" {
        upgrade(noconfirm);
    } else if oper == "-Sy" || oper == "upd" {
        update();
    } else if oper == "-Ss" || oper == "sea" {
        r_search(&args[2]);
        a_search(&args[2]);
    } else if oper == "-Sa" || oper == "aursea" {
        a_search(&args[2]);
    } else if oper == "-Sr" || oper == "repsea" {
        r_search(&args[2]);
    } else if oper == "-Cc" || oper == "clr" {
        clearcache();
    } else if oper == "-v" || oper == "-V" || oper == "ver" {
        ver();
    } else if oper == "-h" || oper == "help" {
        help();
    } else {
        let pass = Command::new("pacman")
            .args(env::args().skip(1))
            .status()
            .expect("Something has gone wrong.");

        match pass.code() {
            Some(1) => {
                err_rec(format!("No such operation \"{}\"", args.join(" ")));
                inf(format!(
                    "Try running \"ame help\" for an overview of how to use ame"
                ))
            }
            Some(_) => {}
            None => err_unrec(format!("Something has gone terribly wrong.")),
        }
    }
}
