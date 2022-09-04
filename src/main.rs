use args::{Args, GenCompArgs, InfoArgs};
use builder::pacman::{PacmanColor, PacmanQueryBuilder};
use clap::Parser;

use internal::commands::ShellCommand;
use internal::error::SilentUnwrap;

use crate::args::{InstallArgs, Operation, QueryArgs, RemoveArgs, SearchArgs};
use crate::interact::page_string;
use crate::internal::detect;
use crate::internal::exit_code::AppExitCode;
use crate::internal::{sort, start_sudoloop, structs::Options};
use crate::logging::get_logger;
use crate::logging::Printable;

use clap_complete::{Generator, Shell};
use std::str::FromStr;

mod args;
mod builder;
mod interact;
mod internal;
mod logging;
mod operations;
use logging::init_logger;

#[tokio::main]
async fn main() {
    color_eyre::install().unwrap();
    if unsafe { libc::geteuid() } == 0 {
        crash!( AppExitCode::RunAsRoot, "Running amethyst as root is disallowed as it can lead to system breakage. Instead, amethyst will prompt you when it needs superuser permissions");
    }

    let args: Args = Args::parse();
    init_logger(args.verbose.into());

    let noconfirm = args.no_confirm;

    let options = Options {
        noconfirm,
        asdeps: false,
    };

    if args.sudoloop {
        start_sudoloop().await;
    }

    match args.subcommand.unwrap_or_default() {
        Operation::Install(install_args) => cmd_install(install_args, options).await,
        Operation::Remove(remove_args) => cmd_remove(remove_args, options).await,
        Operation::Search(search_args) => cmd_search(search_args, options).await,
        Operation::Query(query_args) => cmd_query(query_args).await,
        Operation::Upgrade(upgrade_args) => {
            tracing::info!("Performing system upgrade");
            operations::upgrade(upgrade_args, options).await;
        }
        Operation::Clean => {
            tracing::info!("Removing orphaned packages");
            operations::clean(options).await;
        }
        Operation::Info(info_args) => cmd_info(info_args).await,
        Operation::GenComp(gen_args) => cmd_gencomp(&gen_args),
        Operation::Diff => todo!(),
    }

    detect().await;
}

#[tracing::instrument(level = "trace")]
async fn cmd_install(args: InstallArgs, options: Options) {
    let packages = args.packages;
    let sorted = sort(&packages, options).await;

    if !sorted.repo.is_empty() {
        operations::install(sorted.repo, options).await;
    }
    if !sorted.aur.is_empty() {
        operations::aur_install(sorted.aur, options).await;
    }
    if !sorted.nf.is_empty() {
        crash!(
            AppExitCode::PacmanError,
            "Couldn't find packages: {} in repos or the AUR",
            sorted.nf.join(", ")
        );
    }

    let bash_output = ShellCommand::bash()
        .arg("-c")
        .arg("sudo find /etc -name *.pacnew")
        .wait_with_output()
        .await
        .silent_unwrap(AppExitCode::Other)
        .stdout;

    if !bash_output.is_empty() {
        let pacnew_files = bash_output
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(", ");
        tracing::info!("You have .pacnew files in /etc ({pacnew_files}) that you haven't removed or acted upon, it is recommended you do that now" );
    }
}

#[tracing::instrument(level = "trace")]
async fn cmd_remove(args: RemoveArgs, options: Options) {
    let packages = args.packages;
    tracing::info!("Uninstalling packages: {}", &packages.join(", "));
    operations::uninstall(packages, options).await;
}

#[tracing::instrument(level = "trace")]
async fn cmd_search(args: SearchArgs, options: Options) {
    let query_string = args.search;

    let mut results = Vec::new();

    let both = !args.aur && !args.repo;
    let aur = args.aur || both;
    let repo = args.repo || both;

    if repo {
        tracing::info!("Searching repos for {}", &query_string);
        let res = operations::search(&query_string, options).await;
        results.extend(res);
    }
    if aur {
        tracing::info!("Searching AUR for {}", &query_string);
        let res = operations::aur_search(&query_string, args.by, options).await;
        results.extend(res);
    }

    if results.is_empty() {
        tracing::info!("No results found");
    } else {
        tracing::info!("Results:");
        let list: Vec<String> = results.iter().map(|x| x.to_print_string()).collect();
        get_logger().print_list(&list, "\n", 0);
        if list.join("\n").lines().count() > crossterm::terminal::size().unwrap().1 as usize {
            page_string(&list.join("\n")).silent_unwrap(AppExitCode::Other);
        }
    }
}

#[tracing::instrument(level = "trace")]
async fn cmd_query(args: QueryArgs) {
    if args.repo || !args.aur {
        tracing::info!("Installed Repo Packages: ");
        PacmanQueryBuilder::native()
            .color(PacmanColor::Always)
            .query()
            .await
            .silent_unwrap(AppExitCode::PacmanError);
    }
    if args.aur || !args.repo {
        tracing::info!("Installed AUR Packages: ");
        PacmanQueryBuilder::foreign()
            .color(PacmanColor::Always)
            .query()
            .await
            .silent_unwrap(AppExitCode::PacmanError);
    }
}

#[tracing::instrument(level = "trace")]
async fn cmd_info(args: InfoArgs) {
    PacmanQueryBuilder::info()
        .package(args.package)
        .query()
        .await
        .silent_unwrap(AppExitCode::PacmanError);
}

#[tracing::instrument(level = "trace")]
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
