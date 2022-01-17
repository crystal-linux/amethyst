mod aur_install;
mod install;
mod query;
mod uninstall;

pub fn install(a: Vec<String>, verbosity: i32) {
    install::install(a, verbosity);
}

pub fn aur_install(a: Vec<String>, verbosity: i32) {
    aur_install::aur_install(a, verbosity);
}
