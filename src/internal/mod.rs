mod clean;
pub mod rpc;
mod sort;
pub mod structs;

pub fn sort(a: &[String], verbosity: i32) -> structs::Sorted {
    sort::sort(a, verbosity)
}

pub fn clean(a: &[String], verbosity: i32) -> Vec<String> {
    clean::clean(a, verbosity)
}
