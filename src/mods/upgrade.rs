use runas::Command;

pub fn upgrade(noconfirm: bool) {
    let errstr = format!("Something happened");
    if noconfirm == true {
        Command::new("pacman")
            .arg("-Syu")
            .arg("--noconfirm")
            .status()
            .expect(&errstr);
    } else {
        Command::new("pacman")
            .arg("-Syu")
            .status()
            .expect(&errstr);
    }
}
