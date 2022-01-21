use crate::Options;

mod clean;
mod initialise;
pub mod rpc;
mod sort;
mod strings;
pub mod structs;

pub fn sort(a: &[String], options: Options) -> structs::Sorted {
    sort::sort(a, options)
}

pub fn clean(a: &[String], options: Options) -> Vec<String> {
    clean::clean(a, options)
}

pub fn init(options: Options) {
    initialise::init(options);
}

pub fn info(a: String) {
    strings::info(a);
}

pub fn crash(a: String, b: i32) {
    strings::crash(a, b);
}

pub fn log(a: String) {
    strings::log(a);
}

pub fn prompt(a: String, b: bool) -> bool {
    strings::prompt(a, b)
}

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
