mod clean;
mod commands;
mod initialise;
pub mod rpc;
mod sort;
mod strings;
pub mod structs;

pub use clean::*;
pub use commands::*;
pub use initialise::*;
pub use sort::*;
use std::env;
pub use strings::*;

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

fn uwu_enabled() -> bool {
    env::var("AME_UWU").unwrap_or_else(|_| "".to_string()) == "true"
}

fn uwu_debug_enabled() -> bool {
    env::var("AME_UWU_DEBUG").unwrap_or_else(|_| "".to_string()) == "true"
}
