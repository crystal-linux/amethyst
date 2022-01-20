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
