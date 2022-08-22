use crate::internal::commands::ShellCommand;
use crate::internal::error::SilentUnwrap;
use crate::internal::exit_code::AppExitCode;
use crate::internal::rpc::rpcsearch;
use crate::{info, log, Options};

use colored::Colorize;

#[allow(clippy::module_name_repetitions)]
pub fn aur_search(query: &str, options: Options) {
    // Initialise variables
    let verbosity = options.verbosity;

    // Query AUR for package info
    let res = rpcsearch(query);

    // Format output
    for package in &res.results {
        println!(
            "{}{} {} {}\n    {}",
            "aur/".cyan().bold(),
            package.name.bold(),
            package.version.green().bold(),
            if package.out_of_date.is_some() {
                "[out of date]".red().bold()
            } else {
                "".bold()
            },
            package
                .description
                .as_ref()
                .unwrap_or(&"No description".to_string())
        );
    }

    if res.results.is_empty() {
        info!("No results found for \"{}\" in the AUR", query);
    }

    if verbosity >= 1 {
        log!("Found {} resuls for \"{}\" in AUR", res.resultcount, query);
    }
}

#[allow(clippy::module_name_repetitions)]
pub fn repo_search(query: &str, options: Options) {
    // Initialise variables
    let verbosity = options.verbosity;

    // Query pacman for package info
    let output = ShellCommand::pacman()
        .arg("-Ss")
        .arg(query)
        .wait_with_output()
        .silent_unwrap(AppExitCode::PacmanError)
        .stdout;

    if verbosity >= 1 {
        log!(
            "Found {} results for \"{}\" in repos",
            &output.split('\n').count() / 2,
            &query
        );
    }

    if output.trim().is_empty() {
        info!("No results found for \"{}\" in the repos", query);
    } else {
        println!("{}", output.trim());
    }
}
