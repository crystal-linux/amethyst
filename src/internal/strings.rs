use std::io;
use std::io::Write;
use std::process::exit;
use std::time::UNIX_EPOCH;

use crate::internal::exit_code::AppExitCode;
use crate::{internal, uwu};

pub fn info<S: ToString>(msg: S) {
    let a = msg.to_string();
    let a = if internal::uwu_enabled() { uwu!(&a) } else { a };

    println!("\x1b[2;22;35m❖\x1b[0m \x1b[1;37m{}\x1b[0m", a)
}

pub fn crash<S: ToString>(msg: S, exit_code: AppExitCode) -> ! {
    let a = msg.to_string();
    let a = if internal::uwu_enabled() { uwu!(&a) } else { a };

    println!("\x1b[2;22;31m❌:\x1b[0m \x1b[1;91m{}\x1b[0m", a);
    exit(exit_code as i32);
}

pub fn log<S: ToString>(msg: S) {
    let a = msg.to_string();
    let a = if internal::uwu_enabled() && internal::uwu_debug_enabled() {
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

pub fn prompt<S: ToString>(a: S, b: bool) -> bool {
    let a = a.to_string();
    let default = ["[Y/n]", "[y/N]"];
    let i = if b { 0 } else { 1 };

    let a = if internal::uwu_enabled() { uwu!(&a) } else { a };

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
