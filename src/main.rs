mod clone;
mod uninstall;
mod help;
mod upgrade;
mod search;
use crate::{clone::clone, help::help, uninstall::uninstall, upgrade::upgrade, search::search};
use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        help();
        exit(1);
    }
    let oper = &args[1];
    if oper == "-S" {
        for arg in env::args().skip(2) {
            clone(&arg);
        }
    } else if oper == "-R" {
        for arg in env::args().skip(2) {
            uninstall(&arg);
        }
    } else if oper == "-Syu" {
        upgrade();
    } else if oper == "-Ss" {
        for arg in env::args().skip(2) {
            search(&arg);
        }
    } else {
        help();
        exit(0);
    }
}
