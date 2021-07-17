mod clone;
mod uninstall;
mod help;
use crate::{clone::clone, help::help, uninstall::uninstall};
use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        help();
        exit(0);
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
    }
}
