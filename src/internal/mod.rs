pub mod rpc;
pub mod structs;
mod sort;

pub fn sort(a: &[String], verbosity: i32) -> structs::Sorted {
    sort::sort(a, verbosity)
}


