use std::io;
use std::process::{exit, Command};

use clap::{App, AppSettings, Arg, ArgMatches, ArgSettings, Shell, SubCommand};

use crate::internal::{crash, info, init, log, sort, structs::Options};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod database;
mod internal;
mod operations;

fn main() {
    extern "C" {
        fn geteuid() -> u32;
    }

    if unsafe { geteuid() } == 0 {
        crash("Running amethyst as root is disallowed as it can lead to system breakage. Instead, amethyst will prompt you when it needs superuser permissions".to_string(), 1);
    }

    fn build_app() -> App<'static, 'static> {
        let app = App::new("Amethyst")
            .version(env!("CARGO_PKG_VERSION"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .arg(
                Arg::with_name("verbose")
                    .short("v")
                    .long("verbose")
                    .multiple(true)
                    .set(ArgSettings::Global)
                    .help("Sets the level of verbosity"),
            )
            .arg(
                Arg::with_name("noconfirm")
                    .long("noconfirm")
                    .set(ArgSettings::Global)
                    .help("Complete operation without prompting user"),
            )
            .subcommand(
                SubCommand::with_name("install")
                    .about(
                        "Installs a package from either the AUR or the PacMan-defined repositories",
                    )
                    .aliases(&["-S", "ins"])
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
                    .aliases(&["-R", "-Rs", "rm"])
                    .arg(
                        Arg::with_name("package(s)")
                            .help("The name of the package(s) to remove")
                            .required(true)
                            .multiple(true)
                            .index(1),
                    ),
            )
            .subcommand(
                SubCommand::with_name("search")
                    .about("Searches for the relevant packages in both the AUR and repos")
                    .aliases(&["-Ss", "sea"])
                    .arg(
                        Arg::with_name("aur")
                            .short("a")
                            .long("aur")
                            .help("Search only the AUR for the package"),
                    )
                    .arg(
                        Arg::with_name("repo")
                            .short("r")
                            .long("repo")
                            .help("Searches only local repos for the package"),
                    )
                    .arg(
                        Arg::with_name("package(s)")
                            .help("The name of the package to search for")
                            .required(true)
                            .multiple(false)
                            .index(1),
                    ),
            )
            .subcommand(
                SubCommand::with_name("query")
                    .about("Queries installed packages")
                    .aliases(&["-Q", "ls"])
                    .arg(
                        Arg::with_name("aur")
                            .short("a")
                            .help("Lists AUR/foreign packages"),
                    )
                    .arg(
                        Arg::with_name("repo")
                            .short("r")
                            .help("Lists repo/native packages"),
                    ),
            )
            .subcommand(
                SubCommand::with_name("upgrade")
                    .about("Upgrades locally installed packages to their latest versions")
                    .aliases(&["-Syu", "upg"]),
            )
            .subcommand(
                SubCommand::with_name("compgen")
                    .about("Generates shell completions for given shell (bash by default)")
                    .aliases(&["-G", "cg"])
                    .arg(
                        Arg::with_name("shell")
                            .help("The name of the shell you want to generate completions for")
                            .possible_values(&["bash", "fish", "zsh", "pwsh", "elvish"])
                            .required(true),
                    ),
            )
            .settings(&[
                AppSettings::GlobalVersion,
                AppSettings::VersionlessSubcommands,
                AppSettings::ArgRequiredElseHelp,
                AppSettings::InferSubcommands,
            ]);
        app
    }

    let matches = build_app().get_matches();

    let verbosity: i32 = matches.occurrences_of("verbose") as i32;
    let noconfirm: bool = matches.is_present("noconfirm");

    let options = Options {
        verbosity,
        noconfirm,
        asdeps: false,
    };

    init(options);

    fn collect_matches(a: &ArgMatches) -> Vec<String> {
        a.subcommand()
            .1
            .unwrap()
            .values_of("package(s)")
            .unwrap()
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    }

    if let true = matches.is_present("install") {
        let packages = collect_matches(&matches);
        let sorted = sort(&packages, options);

        info(format!(
            "Attempting to install packages: {}",
            packages.join(", ")
        ));

        if !sorted.repo.is_empty() {
            operations::install(sorted.repo, options);
        }
        if !sorted.aur.is_empty() {
            operations::aur_install(sorted.aur, options);
        }
        if !sorted.nf.is_empty() {
            log(format!(
                "Couldn't find packages: {} in repos or the AUR",
                sorted.nf.join(", ")
            ));
        }
        exit(0);
    }

    if let true = matches.is_present("remove") {
        let packages = collect_matches(&matches);
        info(format!("Uninstalling packages: {}", &packages.join(", ")));
        operations::uninstall(packages, options);
        exit(0);
    }

    if let true = matches.is_present("upgrade") {
        info("Performing system upgrade".to_string());
        operations::upgrade(options);
        exit(0);
    }

    if let true = matches.is_present("search") {
        let packages = collect_matches(&matches);
        if matches
            .subcommand_matches("search")
            .unwrap()
            .is_present("aur")
        {
            info(format!("Searching AUR for {}", &packages[0]));
            operations::aur_search(&packages[0], options);
        }
        if matches
            .subcommand_matches("search")
            .unwrap()
            .is_present("repo")
        {
            info(format!("Searching repos for {}", &packages[0]));
            operations::search(&packages[0], options);
        }

        if !matches
            .subcommand_matches("search")
            .unwrap()
            .is_present("repo")
            && !matches
                .subcommand_matches("search")
                .unwrap()
                .is_present("aur")
        {
            info(format!("Searching AUR and repos for {}", &packages[0]));
            operations::search(&packages[0], options);
            operations::aur_search(&packages[0], options);
        }
        exit(0);
    }

    if let true = matches.is_present("query") {
        if matches
            .subcommand_matches("query")
            .unwrap()
            .is_present("aur")
        {
            Command::new("pacman")
                .arg("-Qm")
                .spawn()
                .expect("Something has gone wrong")
                .wait()
                .unwrap();
        }
        if matches
            .subcommand_matches("query")
            .unwrap()
            .is_present("repo")
        {
            Command::new("pacman")
                .arg("-Qn")
                .spawn()
                .expect("Something has gone wrong")
                .wait()
                .unwrap();
        }
        if !matches
            .subcommand_matches("query")
            .unwrap()
            .is_present("aur")
            && !matches
                .subcommand_matches("query")
                .unwrap()
                .is_present("repo")
        {
            Command::new("pacman")
                .arg("-Qn")
                .spawn()
                .expect("Something has gone wrong")
                .wait()
                .unwrap();
            Command::new("pacman")
                .arg("-Qm")
                .spawn()
                .expect("Something has gone wrong")
                .wait()
                .unwrap();
        }
        exit(0);
    }

    if let true = &matches.is_present("compgen") {
        let mut app = build_app();
        match matches
            .subcommand_matches("compgen")
            .unwrap()
            .value_of("shell")
            .unwrap()
        {
            "bash" => {
                app.gen_completions_to("ame", Shell::Bash, &mut io::stdout());
            }
            "fish" => {
                app.gen_completions_to("ame", Shell::Fish, &mut io::stdout());
            }
            "zsh" => {
                app.gen_completions_to("ame", Shell::Zsh, &mut io::stdout());
            }
            "pwsh" => {
                app.gen_completions_to("ame", Shell::PowerShell, &mut io::stdout());
            }
            "elvish" => {
                app.gen_completions_to("ame", Shell::Elvish, &mut io::stdout());
            }
            _ => {}
        }
    }
}
