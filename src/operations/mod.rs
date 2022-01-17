mod install;
mod uninstall;
mod query;

pub fn install(a: Vec<String>) {
    install::install(a);
}