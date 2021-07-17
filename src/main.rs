mod clone;
mod uninstall;
mod help;
mod upgrade;
use crate::{clone::clone, help::help, uninstall::uninstall, upgrade::upgrade};
use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let oper = &args[1];
    print!("{}", oper);
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
    } else {
        help();
        exit(0);
    }
}
