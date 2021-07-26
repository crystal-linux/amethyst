use runas::Command;

pub fn install(pkg: &str) {
    let errstr = format!("Oops.. Something went wrong!");
    Command::new("pacman").arg("-S").arg(&pkg).status().expect(&errstr);
}
