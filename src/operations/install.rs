pub fn install(a: Vec<String>, verbosity: i32) {
    match verbosity {
        0 => {},
        1 => {
            eprintln!("Installing from repos:");
            eprintln!("{:?}", &a);
        }
        _ => {
            eprintln!("Installing from repos:");
            for b in a {
                eprintln!("{:?}", b);
            }
        }
    }
}