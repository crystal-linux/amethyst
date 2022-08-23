use chrono::{Local, TimeZone};
use colored::Colorize;
use textwrap::wrap;

use crate::internal::commands::ShellCommand;
use crate::internal::error::SilentUnwrap;
use crate::internal::exit_code::AppExitCode;
use crate::internal::rpc::rpcsearch;
use crate::{log, Options};

#[allow(clippy::module_name_repetitions)]
/// Searches for packages from the AUR and returns wrapped results
pub fn aur_search(query: &str, options: Options) -> String {
    // Query AUR for package info
    let res = rpcsearch(query);

    // Get verbosity
    let verbosity = options.verbosity;

    // Format output
    let mut results_vec = vec![];
    for package in &res.results {
        // Define wrapping options
        let opts = textwrap::Options::new(crossterm::terminal::size().unwrap().0 as usize - 4)
            .subsequent_indent("    ");

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
                opts,
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

struct SearchResult {
    repo: String,
    name: String,
    version: String,
    description: String,
}

#[allow(clippy::module_name_repetitions)]
/// Searches for packages from the repos and returns wrapped results
pub fn repo_search(query: &str, options: Options) -> String {
    // Initialise variables
    let verbosity = options.verbosity;

    // Query pacman for package info
    let output = ShellCommand::bash()
        .args(&["-c", &format!("expac -Ss '%r\\\\%n\\\\%v\\\\%d' {}", query)])
        .arg(query)
        .wait_with_output()
        .silent_unwrap(AppExitCode::PacmanError)
        .stdout;

    // Split output into lines
    let lines = output.trim().split('\n');

    // Initialise results vector
    let mut results_vec: Vec<SearchResult> = vec![];

    // Iterate over lines
    for line in lines {
        let parts: Vec<&str> = line.split('\\').collect();
        let res = SearchResult {
            repo: parts[0].to_string(),
            name: parts[1].to_string(),
            version: parts[2].to_string(),
            description: parts[3].to_string(),
        };
        results_vec.push(res);
    }

    if verbosity >= 1 {
        log!(
            "Found {} results for \"{}\" in repos",
            &results_vec.len(),
            &query
        );
    }

    // Format output
    let results_vec = results_vec
        .into_iter()
        .map(|res| {
            let opts = textwrap::Options::new(crossterm::terminal::size().unwrap().0 as usize - 4)
                .subsequent_indent("    ");
            format!(
                "{}{}{} {}\n    {}",
                res.repo.purple().bold(),
                "/".purple().bold(),
                res.name.bold(),
                res.version.green().bold(),
                if res.description.is_empty() {
                    "No description".to_string()
                } else {
                    wrap(&res.description, opts).join("\n")
                },
            )
        })
        .collect::<Vec<String>>();

    if output.trim().is_empty() {
        "".to_string()
    } else {
        results_vec.join("\n")
    }
}
