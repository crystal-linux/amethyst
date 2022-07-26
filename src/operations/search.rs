use crate::internal::commands::ShellCommand;
use crate::internal::error::SilentUnwrap;
use crate::internal::exit_code::AppExitCode;
use crate::internal::rpc::rpcsearch;
use crate::{log, Options};

pub fn aur_search(query: &str, options: Options) {
    // Initialise variables
    let verbosity = options.verbosity;

    // Query AUR for package info
    let res = rpcsearch(query.to_string());

    // Format output
    for package in &res.results {
        println!(
            "aur/{} {}\n    {}",
            package.name,
            package.version,
            package
                .description
                .as_ref()
                .unwrap_or(&"No description".to_string())
        )
    }

    if verbosity >= 1 {
        log!("Found {} resuls for \"{}\" in AUR", res.resultcount, query);
    }
}

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

    println!("{}", output)
}
