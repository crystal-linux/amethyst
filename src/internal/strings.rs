use std::io::Write;
use std::process::exit;
use std::str::FromStr;
use std::time::UNIX_EPOCH;
use std::{env, io};

use crate::uwu;

pub fn info(a: String) {
    let a = if env::var("AME_UWU").unwrap_or("".to_string()) == "true" {
        uwu!(&a)
    } else {
        a
    };

    println!("\x1b[2;22;35m❖\x1b[0m \x1b[1;37m{}\x1b[0m", a)
}

pub fn crash(a: String, b: i32) {
    let a = if env::var("AME_UWU").unwrap_or("".to_string()) == "true" {
        uwu!(&a)
    } else {
        a
    };

    println!("\x1b[2;22;31m❌\x1b[0m \x1b[1;91m{}\x1b[0m", a);
    exit(b);
}

pub fn log(a: String) {
    let a = if env::var("AME_UWU").unwrap_or("".to_string()) == "true"
        && env::var("AME_UWU_DEBUG").unwrap_or("".to_string()) == "true"
    {
        uwu!(&a)
    } else {
        a
    };

    eprintln!(
        "{} {}",
        std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        a
    );
}

pub fn prompt(a: String, b: bool) -> bool {
    let default = ["[Y/n]", "[y/N]"];
    let i = if b { 0 } else { 1 };

    let a = if env::var("AME_UWU").unwrap_or("".to_string()) == "true" {
        uwu!(&a)
    } else {
        a
    };

    print!(
        "\x1b[2;22;35m?\x1b[0m \x1b[1;37m{}\x1b[0m \x1b[2;22;37m{}\x1b[0m: ",
        a, default[i]
    );

    let mut yn: String = String::new();

    io::stdout().flush().ok();
    let _ = std::io::stdin().read_line(&mut yn);

    if yn.trim().to_lowercase() == "n" || yn.trim().to_lowercase() == "no" {
        false
    } else if yn.trim().to_lowercase() == "y" || yn.trim().to_lowercase() == "yes" {
        true
    } else {
        b
    }
}
