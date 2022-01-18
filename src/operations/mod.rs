use crate::Options;

mod aur_install;
mod install;
mod search;
mod uninstall;

pub fn install(a: Vec<String>, options: Options) {
    install::install(a, options);
}

pub fn uninstall(a: Vec<String>, options: Options) {
    uninstall::uninstall(a, options);
}

pub fn search(a: &str, options: Options) {
    search::repo_search(a, options);
}

pub fn aur_install(a: Vec<String>, options: Options) {
    aur_install::aur_install(a, options);
}

pub fn aur_search(a: &str, options: Options) {
    search::aur_search(a, options);
}
