mod internal;
mod operations;

use crate::internal::sort;
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
            SubCommand::with_name("install")
                .about("Installs a package from either the AUR or the PacMan-defined repositories")
                .aliases(&["-S", "ins"])
                .arg(
                    Arg::with_name("noconfirm")
                        .short("y")
                        .long("noconfirm")
                        .help("Do not ask for confirmation before installing packages"),
                )
                .arg(
                    Arg::with_name("package(s)")
                        .help("The name of the package(s) to install")
                        .required(true)
                        .multiple(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("remove")
                .about("Removes a previously installed package")
                .aliases(&["-R", "rm"])
                .arg(
                    Arg::with_name("noconfirm")
                        .short("y")
                        .long("noconfirm")
                        .help("Do not ask for confirmation before removing packages"),
                )
                .arg(
                    Arg::with_name("recursive")
                        .short("s")
                        .long("recursive")
                        .help("Recursively uninstall orphaned dependencies"),
                )
                .arg(
                    Arg::with_name("package(s)")
                        .help("The name of the package(s) to remove")
                        .required(true)
                        .multiple(true)
                        .index(1),
                ),
        )
        .get_matches();

    let verbosity: i32 = matches.occurrences_of("verbose") as i32;

    let packages: Vec<String> = matches
        .subcommand_matches("install")
        .unwrap()
        .values_of("package(s)")
        .unwrap()
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    if let true = matches.is_present("install") {
        let sorted = sort(&packages, verbosity);
        operations::install(sorted.repo, verbosity);
    }
}
