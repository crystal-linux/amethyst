use args::{Args, GenCompArgs};
use builder::pacman::{PacmanColor, PacmanQueryBuilder};
use clap::Parser;

use internal::commands::ShellCommand;
use internal::detect;
use internal::error::SilentUnwrap;

use crate::args::{InstallArgs, Operation, QueryArgs, RemoveArgs};
use crate::interact::page_string;
use crate::internal::config::Config;
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
        crash!(AppExitCode::RunAsRoot, "{}", fl!("run-as-root"));
    }

    let args: Args = Args::parse();
    init_logger(args.verbose.into());

    let noconfirm = args.no_confirm;
    let quiet = args.quiet;

    let options = Options {
        noconfirm,
        quiet,
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
            tracing::info!("{}", fl!("system-upgrade"));
            operations::upgrade(upgrade_args, options).await;
        }
        Operation::Clean => {
            tracing::info!("{}", fl!("removing-orphans"));
            operations::clean(options).await;
        }
        Operation::GenComp(gen_args) => cmd_gencomp(&gen_args),
        Operation::Diff => detect().await,
    }
}

#[tracing::instrument(level = "trace")]
async fn cmd_install(args: InstallArgs, options: Options) {
    let packages = &args.packages;
    let both = !args.aur && !args.repo;
    let noconfirm = options.noconfirm;

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
                        "{}",
                        fl!("couldnt-find-packages", packages = sorted.nf.join(", "))
                    );
                }
                if !sorted.repo.is_empty() {
                    operations::install(sorted.repo, options).await;
                }
                if !sorted.aur.is_empty() {
                    if Config::read().base.aur_verification_prompt {
                        tracing::info!("{}", fl!("following-packages"));
                        get_logger().print_list(&sorted.aur, "  ", 2);
                        newline!();
                        tracing::warn!("{}", fl!("aur-warning"));
                        let cont = noconfirm || prompt!(default no, "{}", fl!("are-you-sure"));
                        if !cont {
                            tracing::info!("{}", fl!("exiting"));
                            std::process::exit(AppExitCode::PacmanError as i32);
                        }
                    }
                    operations::aur_install(sorted.aur, options).await;
                }
            }
        }
    };
}

#[tracing::instrument(level = "trace")]
async fn cmd_remove(args: RemoveArgs, options: Options) {
    let packages = args.packages;
    tracing::info!(
        "{}",
        fl!("uninstalling-packages", packages = packages.join(", "))
    );
    operations::uninstall(packages, options).await;
}

#[tracing::instrument(level = "trace")]
async fn cmd_search(args: InstallArgs, options: Options) {
    let query_string = args.packages.join(" ");
    let both = !args.aur && !args.repo;

    let mut results = Vec::new();

    if args.repo || both {
        tracing::info!("{}", fl!("searching-repos", query = query_string.clone()));
        let res = operations::search(&query_string, options).await;
        results.extend(res);
    }
    if args.aur || both {
        tracing::info!("{}", fl!("searching-aur", query = query_string.clone()));
        let res = operations::aur_search(&query_string, args.by, options).await;
        results.extend(res);
    }

    if results.is_empty() {
        tracing::info!("{}", fl!("no-results"));
    } else {
        tracing::info!("{}", fl!("results"));

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
        tracing::info!("{}", fl!("installed-repo-packages"));
        PacmanQueryBuilder::native()
            .color(PacmanColor::Always)
            .query()
            .await
            .silent_unwrap(AppExitCode::PacmanError);
    }

    if args.aur {
        tracing::info!("{}", fl!("installed-aur-packages"));
        PacmanQueryBuilder::foreign()
            .color(PacmanColor::Always)
            .query()
            .await
            .silent_unwrap(AppExitCode::PacmanError);
    }

    if both {
        tracing::info!("{}", fl!("installed-packages"));
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
            crash!(AppExitCode::Other, "{}", fl!("invalid-shell", shell = e));
        });

        if shell == Shell::Zsh {
            crash!(AppExitCode::Other, "{}", fl!("zsh-error"));
        };

        shell.generate(
            &<args::Args as clap::CommandFactory>::command(),
            &mut std::io::stderr(),
        );
    }
}
