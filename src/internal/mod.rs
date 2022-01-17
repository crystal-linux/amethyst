pub mod rpc;
pub mod structs;
mod sort;

pub fn sort(a: &[String]) -> structs::Sorted {
    sort::sort(a)
}


