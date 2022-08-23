use crate::internal::commands::ShellCommand;
use crate::internal::error::SilentUnwrap;
use crate::internal::exit_code::AppExitCode;
use crate::internal::rpc::rpcsearch;
use crate::{log, Options};

use chrono::{Local, TimeZone};
use colored::Colorize;
use textwrap::{termwidth, wrap};

#[allow(clippy::module_name_repetitions)]
pub fn aur_search(query: &str, options: Options) -> String {
    // Query AUR for package info
    let res = rpcsearch(query);

    // Get verbosity
    let verbosity = options.verbosity;

    // Format output
    let mut results_vec = vec![];
    for package in &res.results {
        // Define wrapping options
        let opts = textwrap::Options::new(termwidth()).subsequent_indent("    ");

        let result = format!(
            "{}{} {} {}\n    {}",
            "aur/".cyan().bold(),
            package.name.bold(),
            package.version.green().bold(),
            if package.out_of_date.is_some() {
                format!(
                    "[out of date: since {}]",
                    Local
                        .timestamp(package.out_of_date.unwrap().try_into().unwrap(), 0)
                        .date_naive()
                )
                .red()
                .bold()
            } else {
                "".bold()
            },
            wrap(
                package
                    .description
                    .as_ref()
                    .unwrap_or(&"No description".to_string()),
                opts
            )
            .join("\n"),
        );
        results_vec.push(result);
    }

    if verbosity > 1 {
        log!(
            "Found {} results for \"{}\" in the AUR",
            res.results.len(),
            query
        );
    }

    results_vec.join("\n")
}

#[allow(clippy::module_name_repetitions)]
pub fn repo_search(query: &str, options: Options) -> String {
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
        "".to_string()
    } else {
        output.trim().to_string()
    }
}
