mod install;
mod uninstall;
mod query;

pub fn install(a: Vec<String>, verbosity: i32) {
    install::install(a, verbosity);
}