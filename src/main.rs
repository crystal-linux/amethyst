mod mods;
use mods::{clearcache::{clearcache}, clone::clone, help::help, inssort::inssort, install::install, search::{a_search, r_search}, strs::err_rec, strs::err_unrec, strs::inf, uninstall::{uninstall}, update::{update}, upgrade::{upgrade}, ver::ver, xargs::*};
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

    match oper.as_str() {
        "-S" | "-Sn" | "ins" => {
            inssort(noconfirm, pkgs);
        }
        "-R" | "-Rn" | "-Rsn" | "-Rs" | "rm" => {
            uninstall(noconfirm, pkgs);
        }
        "-Syu" | "-Syun" |"upg" => {
            upgrade(noconfirm);
        }
        "-Sy" | "upd" => {
            update();
        }
        "-Ss" | "sea" => {
            r_search(&args[2]);
            a_search(&args[2]);
        }
        "-Sa" | "aursea" => {
            a_search(&args[2]);
        }
        "-Sr" | "repsea" => {
            r_search(&args[2]);
        }
        "-Cc" | "clr" => {
            clearcache();
        }
        "-v" | "-V" | "ver" => {
            ver();
        }
        "-h" | "help" => {
            help()
        }
        _ => {
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
}
