mod mods;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Amethyst")
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(
            SubCommand::with_name ("install")
                .about("Installs a package from either the AUR or the PacMan-defined repositories")
                .arg(
                    Arg::with_name("noconfirm")
                        .short("y")
                        .long("noconfirm")
                        .help("Do not ask for confirmation before installing the package")
                )
                .arg(
                    Arg::with_name("package")
                        .help("The name of the package to install")
                        .required(true)
                        .index(1)
                )    
        )
        .get_matches();

        match matches.occurrences_of("verbose") {
            0 => println!("No verbosity"),
            1 => println!("Some extra information"),
            2 => println!("Plenty of debug text"),
            _ => println!("Screensaver mode"),
        }
}
