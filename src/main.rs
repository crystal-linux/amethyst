use args::Args;
use clap::Parser;
use internal::commands::ShellCommand;
use internal::error::SilentUnwrap;

use crate::args::{InstallArgs, Operation, QueryArgs, RemoveArgs, SearchArgs};
use crate::internal::detect;
use crate::internal::exit_code::AppExitCode;
use crate::internal::{init, sort, start_sudoloop, structs::Options};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod args;
mod database;
mod internal;
mod operations;

fn main() {
    // Break if we are running as root
    if unsafe { libc::geteuid() } == 0 {
        crash!( AppExitCode::RunAsRoot, "Running amethyst as root is disallowed as it can lead to system breakage. Instead, amethyst will prompt you when it needs superuser permissions");
    }

    // Parse arguments
    let args: Args = Args::parse();

    // Initialize variables
    let verbosity = args.verbose as i32;
    let noconfirm = args.no_confirm;

    // Get options struct
    let options = Options {
        verbosity,
        noconfirm,
        asdeps: false,
    };

    // Ensure amethyst is initialized
    init(options);

    // Start sudoloop if specified
    if args.sudoloop {
        start_sudoloop();
    }

    // Match args
    match args.subcommand.unwrap_or_default() {
        Operation::Install(install_args) => cmd_install(install_args, options),
        Operation::Remove(remove_args) => cmd_remove(remove_args, options),
        Operation::Search(search_args) => cmd_search(search_args, options),
        Operation::Query(query_args) => cmd_query(query_args),
        Operation::Upgrade => {
            info!("Performing system upgrade");
            operations::upgrade(options);
        }
        Operation::Clean => {
            info!("Removing orphaned packages");
            operations::clean(options);
        }
    }

    // Check for .pacnew files
    detect();
}

fn cmd_install(args: InstallArgs, options: Options) {
    // Initialise variables
    let packages = args.packages;
    let sorted = sort(&packages, options);

    info!("Attempting to install packages: {}", packages.join(", "));

    if !sorted.repo.is_empty() {
        // If repo packages found, install them
        operations::install(sorted.repo, options);
    }
    if !sorted.aur.is_empty() {
        // If AUR packages found, install them
        operations::aur_install(sorted.aur, options);
    }
    if !sorted.nf.is_empty() {
        // If some packages are not found, crash TODO: this check should happen first
        crash!(
            AppExitCode::PacmanError,
            "Couldn't find packages: {} in repos or the AUR",
            sorted.nf.join(", ")
        );
    }
}

fn cmd_remove(args: RemoveArgs, options: Options) {
    // Initialise variables
    let packages = args.packages;

    info!("Uninstalling packages: {}", &packages.join(", "));

    // Remove packages
    operations::uninstall(packages, options);
}

fn cmd_search(args: SearchArgs, options: Options) {
    // Initialise variables
    let query_string = args.search.join(" ");
    if args.aur {
        info!("Searching AUR for {}", &query_string);

        // Search AUR
        operations::aur_search(&query_string, options);
    }
    if args.repo {
        info!("Searching repos for {}", &query_string);

        // Search repos
        operations::search(&query_string, options);
    }

    if !args.aur && !args.repo {
        info!("Searching AUR and repos for {}", &query_string);

        // If no search type specified, search both
        operations::search(&query_string, options);
        operations::aur_search(&query_string, options);
    }
}

fn cmd_query(args: QueryArgs) {
    if args.aur {
        // If AUR query, query AUR
        ShellCommand::pacman()
            .arg("-Qm")
            .wait_success()
            .silent_unwrap(AppExitCode::PacmanError);
    }
    if args.repo {
        // If repo query, query repos
        ShellCommand::pacman()
            .arg("-Qn")
            .wait_success()
            .silent_unwrap(AppExitCode::PacmanError);
    }
    if !args.repo && !args.aur {
        // If no query type specified, query both
        ShellCommand::pacman()
            .arg("-Qn")
            .wait_success()
            .silent_unwrap(AppExitCode::PacmanError);
        ShellCommand::pacman()
            .arg("-Qm")
            .wait_success()
            .silent_unwrap(AppExitCode::PacmanError);
    }
}
