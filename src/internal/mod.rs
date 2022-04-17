mod clean;
mod initialise;
pub mod rpc;
mod sort;
mod strings;
pub mod structs;

pub use clean::*;
pub use initialise::*;
pub use sort::*;
pub use strings::*;

#[macro_export]
macro_rules! uwu {
    ($x:expr) => {{
        let uwu: String = String::from_str($x).unwrap();

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
