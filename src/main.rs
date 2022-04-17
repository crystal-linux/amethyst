use clap::Parser;

use crate::args::{InstallArgs, Operation, QueryArgs, RemoveArgs, SearchArgs};
use args::Args;

use crate::internal::{bash, crash, info, init, log, pacman, sort, structs::Options};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod args;
mod database;
mod error;
mod internal;
mod operations;

fn main() {
    extern "C" {
        fn geteuid() -> u32;
    }

    if unsafe { geteuid() } == 0 {
        crash("Running amethyst as root is disallowed as it can lead to system breakage. Instead, amethyst will prompt you when it needs superuser permissions".to_string(), 1);
    }

    let args: Args = Args::parse();

    let verbosity = args.verbose as i32;
    let noconfirm = args.no_confirm;

    let options = Options {
        verbosity,
        noconfirm,
        asdeps: false,
    };

    init(options);

    match args.subcommand.unwrap_or_default() {
        Operation::Install(install_args) => cmd_install(install_args, options),
        Operation::Remove(remove_args) => cmd_remove(remove_args, options),
        Operation::Search(search_args) => cmd_search(search_args, options),
        Operation::Query(query_args) => cmd_query(query_args),
        Operation::Upgrade => {
            info("Performing system upgrade".to_string());
            operations::upgrade(options);
        }
    }
}

fn cmd_install(args: InstallArgs, options: Options) {
    let packages = args.packages;
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

    let bash_output = bash(&["-c", "sudo find /etc -name *.pacnew"]).unwrap();

    if !bash_output.is_empty() {
        let pacnew_files = bash_output
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(", ");
        info(format!("You have .pacnew files in /etc ({pacnew_files}) that you haven't removed or acted upon, it is recommended you do that now", ));
    }
}

fn cmd_remove(args: RemoveArgs, options: Options) {
    let packages = args.packages;
    info(format!("Uninstalling packages: {}", &packages.join(", ")));
    operations::uninstall(packages, options);
}

fn cmd_search(args: SearchArgs, options: Options) {
    let query_string = args.search.join(" ");
    if args.aur {
        info(format!("Searching AUR for {}", &query_string));
        operations::aur_search(&query_string, options);
    }
    if args.repo {
        info(format!("Searching repos for {}", &query_string));
        operations::search(&query_string, options);
    }

    if !args.aur && !args.repo {
        info(format!("Searching AUR and repos for {}", &query_string));
        operations::search(&query_string, options);
        operations::aur_search(&query_string, options);
    }
}

fn cmd_query(args: QueryArgs) {
    if args.aur {
        pacman(&["-Qm"]).unwrap();
    }
    if args.repo {
        pacman(&["-Qn"]).unwrap();
    }
    if !args.repo && !args.aur {
        pacman(&["-Qn"]).unwrap();
        pacman(&["-Qm"]).unwrap();
    }
}
