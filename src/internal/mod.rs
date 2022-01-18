use crate::Options;

mod clean;
pub mod rpc;
mod sort;
pub mod structs;

pub fn sort(a: &[String], options: Options) -> structs::Sorted {
    sort::sort(a, options)
}

pub fn clean(a: &[String], options: Options) -> Vec<String> {
    clean::clean(a, options)
}
