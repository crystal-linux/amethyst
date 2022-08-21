pub use clean::*;
pub use clean::*;
pub use detect::*;
pub use initialise::*;
pub use initialise::*;
pub use sort::*;
pub use sort::*;
use std::env;
pub use sudoloop::*;

mod clean;
pub mod commands;
mod detect;
pub mod error;
pub mod exit_code;
mod initialise;
pub mod rpc;
mod sort;
pub mod structs;
#[macro_use]
pub mod utils;
mod sudoloop;

#[macro_export]
macro_rules! uwu {
    ($x:expr) => {{
        let uwu: String = String::from($x);

        let uwu = uwu.replace("l", "w");
        let uwu = uwu.replace("L", "W");
        let uwu = uwu.replace("r", "w");
        let uwu = uwu.replace("R", "W");
        let uwu = uwu.replace("na", "nya");
        let uwu = uwu.replace("Na", "Nya");
        let uwu = uwu.replace("NA", "NYA");

        uwu
    }};
}

pub fn uwu_enabled() -> bool {
    env::var("AME_UWU").unwrap_or_else(|_| "".to_string()) == "true"
}

pub fn uwu_debug_enabled() -> bool {
    env::var("AME_UWU_DEBUG").unwrap_or_else(|_| "".to_string()) == "true"
}

/// Checks if we're running in a tty. If we do we can assume that
/// the output can safely be colorized.
pub fn is_tty() -> bool {
    (unsafe { libc::isatty(libc::STDIN_FILENO as i32) } != 0)
}
