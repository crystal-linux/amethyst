use runas::Command;

pub fn install(pkg: &str) {
    let errstr = format!("Something went wrong");
    Command::new("pacman").arg("-S").arg(&pkg).status().expect(&errstr);
}
