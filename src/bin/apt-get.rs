use std::env;

fn main() {
    let arg = &env::args().collect::<Vec<String>>()[0];

    println!(
        "Sorry for the bother, we don't use \x1b[2;22;35m{}\x1b[0m on Crystal, we use \x1b[2;22;35mame\x1b[0m! Please use that instead!",
        arg.split('/')
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
    );
    std::process::exit(0);
}
