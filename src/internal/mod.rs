pub use clean::*;
pub use clean::*;
pub use detect::*;
pub use sort::*;
pub use sudoloop::*;

mod clean;
pub mod commands;
pub mod config;
pub mod dependencies;
mod detect;
pub mod error;
pub mod exit_code;
pub mod fs_utils;
pub mod rpc;
mod sort;
pub mod structs;
#[macro_use]
pub mod utils;
pub mod alpm;
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
    let config = config::read();
    config.extra.uwu.unwrap_or(false)
}

/// Checks if we're running in a tty. If we do we can assume that
/// the output can safely be colorized.
pub fn is_tty() -> bool {
    (unsafe { libc::isatty(libc::STDIN_FILENO as i32) } != 0)
}
