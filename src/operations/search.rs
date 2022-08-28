use chrono::{Local, TimeZone};
use colored::Colorize;
use textwrap::wrap;

use std::fmt::Display;

use crate::internal::commands::ShellCommand;
use crate::internal::error::SilentUnwrap;
use crate::internal::exit_code::AppExitCode;
use crate::internal::rpc::rpcsearch;
use crate::{log, Options};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub repo: String,
    pub name: String,
    pub version: String,
    pub ood: Option<usize>,
    pub description: String,
}

impl Display for SearchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let opts = textwrap::Options::new(crossterm::terminal::size().unwrap().0 as usize - 4)
            .subsequent_indent("    ");
        let description = wrap(&self.description, opts).join("\n");
        write!(
            f,
            "{}{} {} {}\n    {}",
            if self.repo == "aur" {
                (self.repo.clone() + "/").bold().cyan()
            } else {
                (self.repo.clone() + "/").bold().purple()
            },
            self.name.bold(),
            self.version.bold().green(),
            if self.ood.is_some() {
                format!(
                    "[out of date: since {}]",
                    Local
                        .timestamp(self.ood.unwrap().try_into().unwrap(), 0)
                        .date_naive()
                )
                .bold()
                .red()
            } else {
                "".bold()
            },
            description
        )
    }
}

pub struct ResultsVec(pub Vec<SearchResult>);

impl From<Vec<SearchResult>> for ResultsVec {
    fn from(v: Vec<SearchResult>) -> Self {
        Self(v)
    }
}

impl Display for ResultsVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for result in &self.0 {
            writeln!(f, "{}", result)?;
        }
        Ok(())
    }
}

#[allow(clippy::module_name_repetitions)]
/// Searches for packages from the AUR and returns wrapped results
pub fn aur_search(query: &str, options: Options) -> Vec<SearchResult> {
    // Query AUR for package info
    let res = rpcsearch(query);

    // Get verbosity
    let verbosity = options.verbosity;

    // Format output
    let mut results_vec = vec![];
    for package in &res.results {
        let result = SearchResult {
            repo: "aur".to_string(),
            name: package.name.to_string(),
            version: package.version.to_string(),
            ood: package.out_of_date,
            description: package
                .description
                .as_ref()
                .unwrap_or(&"No description".to_string())
                .to_string(),
        };
        results_vec.push(result);
    }

    if verbosity > 1 {
        log!(
            "Found {} results for \"{}\" in the AUR",
            res.results.len(),
            query
        );
    }

    results_vec
}

#[allow(clippy::module_name_repetitions)]
/// Searches for packages from the repos and returns wrapped results
pub fn repo_search(query: &str, options: Options) -> Vec<SearchResult> {
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
        if line.contains('\\') {
            let parts: Vec<&str> = line.split('\\').collect();
            let res = SearchResult {
                repo: parts[0].to_string(),
                name: parts[1].to_string(),
                version: parts[2].to_string(),
                ood: None,
                description: parts[3].to_string(),
            };
            results_vec.push(res);
        }
    }

    if verbosity >= 1 {
        log!(
            "Found {} results for \"{}\" in repos",
            &results_vec.len(),
            &query
        );
    }

    results_vec
}
