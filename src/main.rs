#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::too_many_lines)]

use args::Args;
use clap::Parser;
use clap_complete::{Generator, Shell};
use internal::commands::ShellCommand;
use internal::error::SilentUnwrap;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use crate::args::{
    GenCompArgs, InfoArgs, InstallArgs, Operation, QueryArgs, RemoveArgs, SearchArgs, UpgradeArgs,
};
use crate::internal::exit_code::AppExitCode;
use crate::internal::utils::pager;
use crate::internal::{detect, init, sort, start_sudoloop, structs::Options};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod args;
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
    let verbosity = args.verbose;
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

    let cachedir = if args.cachedir.is_none() {
        "".to_string()
    } else {
        // Create cache directory if it doesn't exist
        if fs::metadata(&args.cachedir.as_ref().unwrap()).is_err() {
            fs::create_dir(&args.cachedir.as_ref().unwrap()).unwrap_or_else(|err| {
                crash!(
                    AppExitCode::FailedCreatingPaths,
                    "Could not create cache directory: {}",
                    err
                );
            });
        }
        Path::new(&args.cachedir.unwrap())
            .canonicalize()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    };

    // Match args
    match args.subcommand.unwrap_or_default() {
        Operation::Install(install_args) => cmd_install(install_args, options, &cachedir),
        Operation::Remove(remove_args) => cmd_remove(remove_args, options),
        Operation::Search(search_args) => cmd_search(&search_args, options),
        Operation::Query(query_args) => cmd_query(&query_args),
        Operation::Info(info_args) => cmd_info(info_args),
        Operation::Upgrade(upgrade_args) => cmd_upgrade(upgrade_args, options, &cachedir),
        Operation::Clean => {
            info!("Removing orphaned packages");
            operations::clean(options);
        }
        Operation::Diff => {
            info!("Running pacdiff");
            detect();
        }
        Operation::GenComp(gencomp_args) => {
            info!("Generating shell completions for {}. Please pipe `stderr` to a file to get completions as a file, e.g. `ame gencomp fish 2> file.fish`", gencomp_args.shell);
            cmd_gencomp(&gencomp_args);
        }
    }
}

fn cmd_install(args: InstallArgs, options: Options, cachedir: &str) {
    // Initialise variables
    let packages = args.packages;
    let sorted = sort(&packages, options);
    let config = internal::config::read();

    info!("Attempting to install packages: {}", packages.join(", "));

    if !sorted.nf.is_empty() {
        // If some packages are not found, crash
        crash!(
            AppExitCode::PacmanError,
            "Couldn't find packages: {} in repos or the AUR",
            sorted.nf.join(", ")
        );
    }

    if !sorted.repo.is_empty() {
        // If repo packages found, install them
        operations::install(&sorted.repo, options);
    }
    if !sorted.aur.is_empty() {
        // If AUR packages found, install them
        operations::aur_install(sorted.aur, options, cachedir);
    }

    // Show optional dependencies for installed packages
    if packages.len() > 1 && config.base.highlight_optdepends {
        info!("Showing optional dependencies for installed packages");
        for p in packages {
            let out = std::process::Command::new("expac")
                .args(&["-Q", "-l", "\n  ", "  %O", &p])
                .output()
                .unwrap()
                .stdout;
            let out = String::from_utf8(out).unwrap().trim().to_string();
            if !out.is_empty() {
                info!("{}:", p);
                println!("  {}", out);
            }
        }
    }
}

fn cmd_remove(args: RemoveArgs, options: Options) {
    // Initialise variables
    let packages = args.packages;

    info!("Uninstalling packages: {}", &packages.join(", "));

    // Remove packages
    operations::uninstall(&packages, options);
}

fn cmd_search(args: &SearchArgs, options: Options) {
    // Initialise variables
    let query_string = args.search.join(" ");

    // Logic for searching
    let both = !args.repo && !args.aur;
    let repo = args.repo || both;
    let aur = args.aur || both;

    // Start repo spinner
    let repo_results = if repo {
        let rsp = spinner!("Searching repos for {}", query_string);

        // Search repos
        let ret = operations::search(&query_string, options);
        rsp.stop_bold("Repo search complete");

        ret
    } else {
        "".to_string()
    };

    // Start AUR spinner
    let aur_results = if aur {
        let asp = spinner!("Searching AUR for {}", query_string);

        // Search AUR
        let ret = operations::aur_search(&query_string, options);
        asp.stop_bold("AUR search complete");

        ret
    } else {
        "".to_string()
    };

    let results = repo_results + "\n" + &aur_results;

    // Print results either way, so that the user can see the results after they exit `less`
    let text = if internal::uwu_enabled() {
        uwu!(results.trim())
    } else {
        results.trim().to_string()
    };

    println!("{}", text);

    // Check if results are longer than terminal height
    if results.lines().count() > crossterm::terminal::size().unwrap().1 as usize {
        // If so, paginate results
        #[allow(clippy::let_underscore_drop)]
        let _ = pager(&results.trim().to_string());
    }
}

fn cmd_query(args: &QueryArgs) {
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

fn cmd_info(args: InfoArgs) {
    ShellCommand::pacman()
        .arg("-Qi")
        .arg(args.package)
        .wait()
        .silent_unwrap(AppExitCode::PacmanError);
}

fn cmd_upgrade(args: UpgradeArgs, options: Options, cachedir: &str) {
    info!("Performing system upgrade");
    operations::upgrade(options, args, cachedir);
}

fn cmd_gencomp(args: &GenCompArgs) {
    let shell: Shell = Shell::from_str(&args.shell).unwrap_or_else(|e| {
        crash!(AppExitCode::Other, "Invalid shell: {}", e);
    });

    if shell == Shell::Zsh {
        crash!(
            AppExitCode::Other,
            "Zsh shell completions are currently unsupported due to a bug in the clap_completion crate"
        );
    };

    shell.generate(
        &<args::Args as clap::CommandFactory>::command(),
        &mut std::io::stderr(),
    );
}
