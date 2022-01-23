use crate::internal::rpc::Package;
use crate::Options;

mod add;
mod initialise;
mod query;
mod remove;

pub fn add(a: Package, options: Options) {
    add::add(a, options);
}

pub fn remove(a: &str, options: Options) {
    remove::remove(a, options);
}

pub fn init(options: Options) {
    initialise::init(options);
}

pub fn query(options: Options) -> Vec<Package> {
    query::query(options)
}
