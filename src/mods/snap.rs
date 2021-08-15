use runas::Command;

pub fn snap(pkg: &str) {
    let errstr = format!("Oops.. Something went wrong!");
    Command::new("snap").arg("install").arg(&pkg).status().expect(&errstr);
}