use args::{Args, GenCompArgs};
use builder::pacman::{PacmanColor, PacmanQueryBuilder};
use clap::Parser;

use internal::commands::ShellCommand;
use internal::error::SilentUnwrap;

use crate::args::{InstallArgs, Operation, QueryArgs, RemoveArgs};
use crate::interact::page_string;
use crate::internal::exit_code::AppExitCode;
use crate::internal::{sort, start_sudoloop, structs::Options};
use crate::logging::get_logger;
use crate::logging::Printable;

use clap_complete::{Generator, Shell};
use clap_complete_fig::Fig;

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
        upgrade: false,
    };

    if args.sudoloop {
        start_sudoloop().await;
    }

    match args.subcommand.unwrap_or_default() {
        Operation::Install(install_args) => cmd_install(install_args, options).await,
        Operation::Remove(remove_args) => cmd_remove(remove_args, options).await,
        Operation::Search(search_args) => {
            let args = InstallArgs {
                search: true,
                ..search_args
            };

            cmd_install(args, options).await;
        }
        Operation::Query(query_args) => cmd_query(query_args).await,
        Operation::Upgrade(upgrade_args) => {
            tracing::info!("Performing system upgrade");
            operations::upgrade(upgrade_args, options).await;
        }
        Operation::Clean => {
            tracing::info!("Removing orphaned packages");
            operations::clean(options).await;
        }
        Operation::GenComp(gen_args) => cmd_gencomp(&gen_args),
        Operation::Diff => todo!(),
    }
}

#[tracing::instrument(level = "trace")]
async fn cmd_install(args: InstallArgs, options: Options) {
    let packages = &args.packages;
    let both = !args.aur && !args.repo;

    match args.search {
        true => {
            cmd_search(args, options).await;
        }
        false => {
            if args.repo && !args.aur {
                operations::install(packages.to_vec(), options).await;
                return;
            }

            if args.aur && !args.repo {
                operations::aur_install(packages.to_vec(), options).await;
                return;
            }

            if both {
                let sorted = sort(packages, options).await;
                if !sorted.nf.is_empty() {
                    crash!(
                        AppExitCode::PacmanError,
                        "Couldn't find packages: {} in repos or the AUR",
                        sorted.nf.join(", ")
                    );
                }
                if !sorted.repo.is_empty() {
                    operations::install(sorted.repo, options).await;
                }
                if !sorted.aur.is_empty() {
                    operations::aur_install(sorted.aur, options).await;
                }
            }
        }
    };
}

#[tracing::instrument(level = "trace")]
async fn cmd_remove(args: RemoveArgs, options: Options) {
    let packages = args.packages;
    tracing::info!("Uninstalling packages: {}", &packages.join(", "));
    operations::uninstall(packages, options).await;
}

#[tracing::instrument(level = "trace")]
async fn cmd_search(args: InstallArgs, options: Options) {
    let query_string = args.packages.join(" ");
    let both = !args.aur && !args.repo;

    let mut results = Vec::new();

    if args.repo || both {
        tracing::info!("Searching repos for {}", &query_string);
        let res = operations::search(&query_string, options).await;
        results.extend(res);
    }
    if args.aur || both {
        tracing::info!("Searching AUR for {}", &query_string);
        let res = operations::aur_search(&query_string, args.by, options).await;
        results.extend(res);
    }

    if results.is_empty() {
        tracing::info!("No results found");
    } else {
        tracing::info!("Results:");

        results.sort_by(|a, b| {
            let a_score = a.score(&query_string);
            let b_score = b.score(&query_string);

            b_score
                .partial_cmp(&a_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let list: Vec<String> = results.iter().map(|x| x.to_print_string()).collect();
        get_logger().print_list(&list, "\n", 0);

        if list.join("\n").lines().count() > crossterm::terminal::size().unwrap().1 as usize {
            page_string(&list.join("\n")).silent_unwrap(AppExitCode::Other);
        }
    }
}

#[tracing::instrument(level = "trace")]
async fn cmd_query(args: QueryArgs) {
    let both = !args.aur && !args.repo && args.info.is_none();

    if args.repo {
        tracing::info!("Installed Repo Packages: ");
        PacmanQueryBuilder::native()
            .color(PacmanColor::Always)
            .query()
            .await
            .silent_unwrap(AppExitCode::PacmanError);
    }

    if args.aur {
        tracing::info!("Installed AUR Packages: ");
        PacmanQueryBuilder::foreign()
            .color(PacmanColor::Always)
            .query()
            .await
            .silent_unwrap(AppExitCode::PacmanError);
    }

    if both {
        tracing::info!("Installed Packages: ");
        PacmanQueryBuilder::all()
            .color(PacmanColor::Always)
            .query()
            .await
            .silent_unwrap(AppExitCode::PacmanError);
    }

    if let Some(info) = args.info {
        PacmanQueryBuilder::info()
            .package(info)
            .query()
            .await
            .silent_unwrap(AppExitCode::PacmanError);
    }
}

#[tracing::instrument(level = "trace")]
fn cmd_gencomp(args: &GenCompArgs) {
    if args.shell == "fig" {
        clap_complete::generate(
            Fig,
            &mut <args::Args as clap::CommandFactory>::command(),
            "ame",
            &mut std::io::stderr(),
        );
    } else {
        let shell: Shell = Shell::from_str(&args.shell).unwrap_or_else(|e| {
            crash!(AppExitCode::Other, "Invalid shell, {}", e);
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
}
