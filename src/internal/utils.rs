use std::io;
use std::io::Write;
use std::process::exit;
use std::time::UNIX_EPOCH;

use crate::internal::exit_code::AppExitCode;
use crate::{internal, uwu};
use colored::*;

const OK_SYMBOL: &str = "❖";
const ERR_SYMBOL: &str = "❌";
const WARN_SYMBOL: &str = "!";
const PROMPT_SYMBOL: &str = "?";

const PROMPT_YN_DEFAULT_TRUE: &str = "[Y/n]";
const PROMPT_YN_DEFAULT_FALSE: &str = "[y/N]";

#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => {
        $crate::internal::utils::log_info(format!($($arg)+))
    }
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => {
        $crate::internal::utils::log_warn(format!($($arg)+))
    }
}

#[macro_export]
macro_rules! crash {
    ($exit_code:expr, $($arg:tt)+) => {
        $crate::internal::utils::log_and_crash(format!($($arg)+), $exit_code)
    }
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)+) => {
        $crate::internal::utils::log_debug(format!($($arg)+))
    }
}

#[macro_export]
macro_rules! prompt {
    (default $default:expr, $($arg:tt)+) => {
        $crate::internal::utils::prompt_yn(format!($($arg)+), $default)
    }
}

pub fn log_info<S: ToString>(msg: S) {
    let msg = msg.to_string();
    let msg = if internal::uwu_enabled() {
        uwu!(&msg)
    } else {
        msg
    };

    println!("{} {}", OK_SYMBOL.purple(), msg.bold())
}

pub fn log_warn<S: ToString>(msg: S) {
    let msg = msg.to_string();
    let msg = if internal::uwu_enabled() {
        uwu!(&msg)
    } else {
        msg
    };

    println!("{} {}", WARN_SYMBOL.yellow(), msg.bold())
}

pub fn log_and_crash<S: ToString>(msg: S, exit_code: AppExitCode) -> ! {
    let msg = msg.to_string();
    let msg = if internal::uwu_enabled() {
        uwu!(&msg)
    } else {
        msg
    };

    println!("{}: {}", ERR_SYMBOL.red().bold(), msg.red().bold());
    exit(exit_code as i32);
}

pub fn log_debug<S: ToString>(msg: S) {
    let msg = msg.to_string();
    let msg = if internal::uwu_enabled() && internal::uwu_debug_enabled() {
        uwu!(&msg)
    } else {
        msg
    };

    eprintln!(
        "{} {}",
        std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        msg
    );
}

pub fn prompt_yn<S: ToString>(question: S, default_true: bool) -> bool {
    let question = question.to_string();

    let yn_prompt = if default_true {
        PROMPT_YN_DEFAULT_TRUE
    } else {
        PROMPT_YN_DEFAULT_FALSE
    };

    let question = if internal::uwu_enabled() {
        uwu!(&question)
    } else {
        question
    };

    print!(
        "{} {} {}: ",
        PROMPT_SYMBOL.purple(),
        question.bold(),
        yn_prompt
    );

    let mut yn: String = String::new();

    io::stdout().flush().ok();
    let _ = std::io::stdin().read_line(&mut yn);

    if yn.trim().to_lowercase() == "n" || yn.trim().to_lowercase() == "no" {
        false
    } else if yn.trim().to_lowercase() == "y" || yn.trim().to_lowercase() == "yes" {
        true
    } else {
        default_true
    }
}
