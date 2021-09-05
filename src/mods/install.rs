use runas::Command;

pub fn install(noconfirm: bool, pkg: &str) {
    println!("{}",noconfirm);
    let errstr = format!("Oops.. Something went wrong!");
    if noconfirm == false {
        Command::new("pacman").arg("-S").arg(&pkg).status().expect(&errstr);
    } else {
        Command::new("pacman").arg("-S").arg("--noconfirm").arg(&pkg).status().expect(&errstr);
    }
}
