use colored::Colorize;
use std::io;
use std::io::Write;
use std::process::{exit, Command, Stdio};
use std::time::UNIX_EPOCH;
use textwrap::wrap;

use crate::internal::exit_code::AppExitCode;
use crate::{internal, uwu};

const OK_SYMBOL: &str = "❖";
const ERR_SYMBOL: &str = "X";
const WARN_SYMBOL: &str = "!";
const PROMPT_SYMBOL: &str = "?";

const PROMPT_YN_DEFAULT_TRUE: &str = "[Y/n]";
const PROMPT_YN_DEFAULT_FALSE: &str = "[y/N]";

#[macro_export]
/// Macro for printing a message to stdout.
macro_rules! info {
    ($($arg:tt)+) => {
        $crate::internal::utils::log_info(format!($($arg)+))
    }
}

#[macro_export]
/// Macro for printing a warning message non-destructively.
macro_rules! warn {
    ($($arg:tt)+) => {
        $crate::internal::utils::log_warn(format!($($arg)+))
    }
}

#[macro_export]
/// Macro for printing a message and destructively exiting
macro_rules! crash {
    ($exit_code:expr, $($arg:tt)+) => {
        $crate::internal::utils::log_and_crash(format!($($arg)+), $exit_code)
    }
}

#[macro_export]
/// Macro for logging to stderr
macro_rules! log {
    ($($arg:tt)+) => {
        $crate::internal::utils::log_debug(format!($($arg)+))
    }
}

#[macro_export]
/// Macro for prompting the user with a yes/no question.
macro_rules! prompt {
    (default $default:expr, $($arg:tt)+) => {
        $crate::internal::utils::prompt_yn(format!($($arg)+), $default)
    }
}

#[macro_export]
/// Macro for creating a spinner.
macro_rules! spinner {
    ($($arg:tt)+) => {
        $crate::internal::utils::spinner_fn(format!($($arg)+))
    }
}

/// Print a formatted message to stdout.
pub fn log_info(msg: String) {
    let msg = if internal::uwu_enabled() {
        uwu!(&msg)
    } else {
        msg
    };

    let opts = textwrap::Options::new(crossterm::terminal::size().unwrap().0 as usize - 2)
        .subsequent_indent("  ");

    println!(
        "{} {}",
        OK_SYMBOL.purple(),
        wrap(&msg, opts).join("\n").bold()
    );
}

/// Print a non-destructive warning message
pub fn log_warn(msg: String) {
    let msg = if internal::uwu_enabled() {
        uwu!(&msg)
    } else {
        msg
    };

    let opts = textwrap::Options::new(crossterm::terminal::size().unwrap().0 as usize - 2)
        .subsequent_indent("  ");

    println!(
        "{} {}",
        WARN_SYMBOL.yellow(),
        wrap(&msg, opts).join("\n").yellow().bold()
    );
}

/// Logs a message and exits the program with the given exit code.
pub fn log_and_crash(msg: String, exit_code: AppExitCode) -> ! {
    let msg = if internal::uwu_enabled() {
        uwu!(&msg)
    } else {
        msg
    };

    let opts = textwrap::Options::new(crossterm::terminal::size().unwrap().0 as usize - 2)
        .subsequent_indent("  ");

    println!(
        "{} {}",
        ERR_SYMBOL.red().bold(),
        wrap(&msg, opts).join("\n").red().bold()
    );
    exit(exit_code as i32);
}

/// Logs a message to stderr with timestamp
pub fn log_debug(msg: String) {
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

/// Prompts the user for a yes/no answer.
pub fn prompt_yn(question: String, default_true: bool) -> bool {
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

    let opts = textwrap::Options::new(crossterm::terminal::size().unwrap().0 as usize - 2)
        .subsequent_indent("  ");

    print!(
        "{} {} {}: ",
        PROMPT_SYMBOL.purple(),
        wrap(&question, opts).join("\n").bold(),
        yn_prompt
    );

    let mut yn: String = String::new();

    io::stdout().flush().ok();
    io::stdin().read_line(&mut yn).unwrap();

    if yn.trim().to_lowercase() == "n" || yn.trim().to_lowercase() == "no" {
        false
    } else if yn.trim().to_lowercase() == "y" || yn.trim().to_lowercase() == "yes" {
        true
    } else {
        default_true
    }
}

pub struct Spinner {
    spinner: spinoff::Spinner,
}

impl Spinner {
    pub fn stop_bold(self, text: &str) {
        let text = if internal::uwu_enabled() {
            uwu!(text)
        } else {
            text.to_string()
        };

        let opts = textwrap::Options::new(crossterm::terminal::size().unwrap().0 as usize - 2)
            .subsequent_indent("  ");

        let symbol = format!("{}", OK_SYMBOL.purple());
        let text = format!("{}", wrap(&text, opts).join("\n").bold());

        self.spinner.stop_and_persist(&symbol, &text);
    }
}

/// Returns a spinner that can be used to display progress.
pub fn spinner_fn(text: String) -> Spinner {
    let text = if internal::uwu_enabled() {
        uwu!(&text)
    } else {
        text
    };

    let opts = textwrap::Options::new(crossterm::terminal::size().unwrap().0 as usize - 2)
        .subsequent_indent("  ");

    Spinner {
        spinner: spinoff::Spinner::new(
            spinoff::Spinners::Line,
            format!("{}", wrap(&text, opts).join("\n").bold()),
            spinoff::Color::Magenta,
        ),
    }
}

/// Opens a String in `less`.
pub fn pager(text: &String) -> io::Result<()> {
    let text = if internal::uwu_enabled() {
        uwu!(text)
    } else {
        text.to_string()
    };

    let mut pager = Command::new("less")
        .arg("-R")
        .stdin(Stdio::piped())
        .spawn()?;

    let stdin = pager.stdin.as_mut().unwrap();
    stdin.write_all(text.as_bytes())?;
    stdin.flush()?;
    pager.wait()?;

    Ok(())
}
