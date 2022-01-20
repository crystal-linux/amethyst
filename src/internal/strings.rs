use std::process::exit;

pub fn info(a: String) {
    println!("\x1b[2;22;35m❖\x1b[0m \x1b[1;37m{}\x1b[0m", a)
}

pub fn crash(a: String, b: i32) {
    println!("\x1b[2;22;31m❌\x1b[0m \x1b[1;91m{}\x1b[0m", a);
    exit(b);
}
