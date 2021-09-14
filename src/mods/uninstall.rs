use runas::Command;

pub fn uninstall(noconfirm: bool, pkg: &str) {
    let errstr = format!("Could not remove package {}", pkg); //again, we should choose one way to do error messages

    if noconfirm == false {
        Command::new("pacman").arg("-R").arg(&pkg).status().expect(&errstr);
    } else {
        Command::new("pacman").arg("-R").arg("--noconfirm").arg(&pkg).status().expect(&errstr);
    }
}
