use args::Args;
use clap::Parser;
use internal::commands::ShellCommand;
use internal::error::SilentUnwrap;

use crate::args::{InstallArgs, Operation, QueryArgs, RemoveArgs, SearchArgs};
use crate::internal::detect;
use crate::internal::exit_code::AppExitCode;
use crate::internal::{init, sort, start_sudoloop, structs::Options};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::EnvFilter;
use std::str::FromStr;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod args;
mod internal;
mod operations;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    if unsafe { libc::geteuid() } == 0 {
        crash!( AppExitCode::RunAsRoot, "Running amethyst as root is disallowed as it can lead to system breakage. Instead, amethyst will prompt you when it needs superuser permissions");
    }
    init_logger();

    let args: Args = Args::parse();

    let verbosity = args.verbose as i32;
    let noconfirm = args.no_confirm;

    let options = Options {
        verbosity,
        noconfirm,
        asdeps: false,
    };

    init(options);

    if args.sudoloop {
        start_sudoloop().await;
    }

    match args.subcommand.unwrap_or_default() {
        Operation::Install(install_args) => cmd_install(install_args, options).await,
        Operation::Remove(remove_args) => cmd_remove(remove_args, options).await,
        Operation::Search(search_args) => cmd_search(search_args, options).await,
        Operation::Query(query_args) => cmd_query(query_args).await,
        Operation::Upgrade => {
            info!("Performing system upgrade");
            operations::upgrade(options).await;
        }
        Operation::Clean => {
            info!("Removing orphaned packages");
            operations::clean(options).await;
        }
    }

    detect().await;
}

/// Initializes the tracing logger
/// Can be used for debug purposes _or_ verbose output
fn init_logger() {
    const DEFAULT_ENV_FILTER: &str = "warn";
    let filter_string =
        std::env::var("AME_LOG").unwrap_or_else(|_| DEFAULT_ENV_FILTER.to_string());
    let env_filter =
        EnvFilter::from_str(&*filter_string).expect("failed to parse env filter string");
    tracing_subscriber::fmt::SubscriberBuilder::default()
        .with_env_filter(env_filter)
        .with_writer(std::io::stdout)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .compact()
        .init();
}

async fn cmd_install(args: InstallArgs, options: Options) {
    let packages = args.packages;
    let sorted = sort(&packages, options).await;

    info!("Attempting to install packages: {}", packages.join(", "));

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
        info!("You have .pacnew files in /etc ({pacnew_files}) that you haven't removed or acted upon, it is recommended you do that now" );
    }
}

async fn cmd_remove(args: RemoveArgs, options: Options) {
    let packages = args.packages;
    info!("Uninstalling packages: {}", &packages.join(", "));
    operations::uninstall(packages, options).await;
}

async fn cmd_search(args: SearchArgs, options: Options) {
    let query_string = args.search.join(" ");
    if args.aur {
        info!("Searching AUR for {}", &query_string);
        operations::aur_search(&query_string, options).await;
    }
    if args.repo {
        info!("Searching repos for {}", &query_string);
        operations::search(&query_string, options).await;
    }

    if !args.aur && !args.repo {
        info!("Searching AUR and repos for {}", &query_string);
        operations::search(&query_string, options).await;
        operations::aur_search(&query_string, options).await;
    }
}

async fn cmd_query(args: QueryArgs) {
    if args.aur {
        ShellCommand::pacman()
            .arg("-Qm")
            .wait_success()
            .await
            .silent_unwrap(AppExitCode::PacmanError);
    }
    if args.repo {
        ShellCommand::pacman()
            .arg("-Qn")
            .wait_success()
            .await
            .silent_unwrap(AppExitCode::PacmanError);
    }
    if !args.repo && !args.aur {
        ShellCommand::pacman()
            .arg("-Qn")
            .wait_success()
            .await
            .silent_unwrap(AppExitCode::PacmanError);
        ShellCommand::pacman()
            .arg("-Qm")
            .wait_success()
            .await
            .silent_unwrap(AppExitCode::PacmanError);
    }
}
