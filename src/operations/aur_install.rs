use crate::Options;

pub fn aur_install(a: Vec<String>, options: Options) {
    let verbosity = options.verbosity;
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
