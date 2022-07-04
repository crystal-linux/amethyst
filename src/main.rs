use clap::Parser;

use crate::args::{InstallArgs, Operation, QueryArgs, RemoveArgs, SearchArgs};
use args::Args;
use internal::commands::ShellCommand;
use internal::error::SilentUnwrap;

use crate::internal::exit_code::AppExitCode;
#[allow(unused_imports)]
use crate::internal::{crash, warn, prompt, info, init, log, sort, structs::Options};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod args;
mod database;
mod internal;
mod operations;

fn main() {
    if unsafe { libc::geteuid() } == 0 {
        crash("Running amethyst as root is disallowed as it can lead to system breakage. Instead, amethyst will prompt you when it needs superuser permissions".to_string(), AppExitCode::RunAsRoot);
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
        crash(
            format!(
                "Couldn't find packages: {} in repos or the AUR",
                sorted.nf.join(", ")
            ),
            AppExitCode::PacmanError,
        );
    }

    let bash_output = ShellCommand::bash()
        .arg("-c")
        .arg("sudo find /etc -name *.pacnew")
        .wait_with_output()
        .silent_unwrap(AppExitCode::Other)
        .stdout;

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
        ShellCommand::pacman()
            .arg("-Qm")
            .wait_success()
            .silent_unwrap(AppExitCode::PacmanError);
    }
    if args.repo {
        ShellCommand::pacman()
            .arg("-Qn")
            .wait_success()
            .silent_unwrap(AppExitCode::PacmanError);
    }
    if !args.repo && !args.aur {
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
