#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::too_many_lines)]

use args::Args;
use clap::Parser;
use internal::commands::ShellCommand;
use internal::error::SilentUnwrap;
use std::fs;
use std::path::Path;

use crate::args::{
    InfoArgs, InstallArgs, Operation, QueryArgs, RemoveArgs, SearchArgs, UpgradeArgs,
};
use crate::internal::exit_code::AppExitCode;
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
    }
}

fn cmd_install(args: InstallArgs, options: Options, cachedir: &str) {
    // Initialise variables
    let packages = args.packages;
    let sorted = sort(&packages, options);

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
        operations::aur_install(sorted.aur.clone(), options, cachedir);
    }

    // Show optional dependencies for installed packages
    if packages.len() > 1 {
        info!("Showing optional dependencies for installed packages");
        for r in sorted.repo {
            info!("{}:", r);
            std::process::Command::new("expac")
                .args(&["-S", "-l", "\n  ", "  %O", &r])
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
        }
        for a in sorted.aur {
            info!("{}:", a);
            let dir_bytes = std::process::Command::new("mktemp")
                .arg("-d")
                .output()
                .unwrap()
                .stdout;
            let dir = String::from_utf8(dir_bytes).unwrap();
            std::process::Command::new("bash")
                .arg("-c")
                .arg(format!("\
                    cd {}
                    curl -L https://aur.archlinux.org/cgit/aur.git/plain/PKGBUILD?h={} -o PKGBUILD -s
                    source PKGBUILD
                    printf '  %s\\n' \"${{optdepends[@]}}\"\
                ", dir, a))
                .stderr(std::process::Stdio::piped())
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
            std::fs::remove_dir_all(&std::path::Path::new(&dir.trim())).unwrap();
        }
    }
}

fn cmd_remove(args: RemoveArgs, options: Options) {
    // Initialise variables
    let packages = args.packages;

    info!("Uninstalling packages: {}", &packages.join(", "));

    // Remove packages
    operations::uninstall(packages, options);
}

fn cmd_search(args: &SearchArgs, options: Options) {
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
