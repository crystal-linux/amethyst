mod mods;
use mods::{
    clearcache::clearcache,
    clone::clone,
    database::create_database,
    help::help,
    inssort::{inssort, inssort_from_file},
    install::install,
    purge::{purge, purge_from_file},
    search::{a_search, r_search},
    strs::err_rec,
    strs::err_unrec,
    strs::inf,
    uninstall::{uninstall, uninstall_from_file},
    update::update,
    upgrade::upgrade,
    ver::ver,
    xargs::*,
    statpkgs::*,
    stat_database::*,
};
use std::{env, process::exit};

fn main() {

    extern "C" {
        fn geteuid() -> u32;
    }

    if unsafe { geteuid() } == 0 {
        // check if user runs ame as root
        err_unrec(
            "Do not run ame as root! this can cause serious damage to your system!".to_string(),
        );
    }

    let args: Vec<String> = env::args().skip(1).collect();
    let mut pkgs: Vec<String> = env::args().skip(2).collect();

    if args.is_empty() {
        help();
        exit(1);
    }

    let file = format!("{}/.local/share/ame/aur_pkgs.db", env::var("HOME").unwrap());
    if !std::path::Path::new(&file).exists() {
        create_database();
    }

    let oper = &args[0];
    let noconfirm: bool = noconf(&args);

    argssort(&mut pkgs);
    match oper.as_str() {
        // match oper
        "-S" | "-Sn" | "ins" => {
            inssort(noconfirm, false, pkgs); // install
        }
        "-Sl" | "-Sln" | "insl" => {
            inssort_from_file(noconfirm, false, &pkgs[0]); // install from file
        }
        "-B" | "-Bn" | "build" => {
            rebuild(noconfirm); // install as a dependency
        }
        "-R" | "-Rn" | "rm" => {
            uninstall(noconfirm, pkgs); // uninstall
        }
        "-Rs" | "-Rsn" | "purge" => {
            purge(noconfirm, pkgs); // purge
        }
        "-Rl" | "-Rln" | "rml" => {
            uninstall_from_file(noconfirm, &pkgs[0]); // uninstall from file
        }
        "-Rsl" | "-Rsln" | "purgel" => {
            purge_from_file(noconfirm, &pkgs[0]); // purge from file
        }
        "-Syu" | "-Syun" | "upg" => {
            upgrade(noconfirm); // upgrade
        }
        "-Sy" | "upd" => {
            update(); // update
        }
        "-Ss" | "sea" => {
            r_search(&args[1]); // search for packages in the repository
            a_search(&args[1]); // search for packages in the aur
        }
        "-Sa" | "aursea" => {
            a_search(&args[1]); // search for packages in the aur
        }
        "-Sr" | "repsea" => {
            r_search(&args[1]); // search for packages in the repository
        }
        "-Cc" | "clr" => {
            clearcache(); // clear cache
        }
        "-v" | "-V" | "ver" => {
            ver(); // version
        }
        "-h" | "help" => {
            help(); // help
        }
        _ => {
            // if oper is not valid it either passes the args to pacman or prints an error
            let pass = runas::Command::new("pacman")
                .args(&args)
                .status()
                .expect("Something has gone wrong.");

            match pass.code() {
                Some(1) => {
                    err_rec(format!("No such operation \"{}\"", args.join(" ")));
                    inf("Try running \"ame help\" for an overview of how to use ame".to_string())
                }
                Some(_) => {}
                None => err_unrec("Something has gone terribly wrong.".to_string()),
            }
        }
    }
}
