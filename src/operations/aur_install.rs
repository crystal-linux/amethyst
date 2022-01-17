pub fn aur_install(a: Vec<String>, verbosity: i32) {
    match verbosity {
        0 => {}
        1 => {
            eprintln!("Installing from AUR:");
            eprintln!("{:?}", &a);
        }
        _ => {
            eprintln!("Installing from AUR:");
            for b in a {
                eprintln!("{:?}", b);
            }
        }
    }
}
