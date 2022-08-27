use crate::internal::commands::ShellCommand;
use crate::internal::error::SilentUnwrap;
use crate::internal::exit_code::AppExitCode;
use crate::internal::rpc::rpcsearch;
use crate::{log, Options};

#[tracing::instrument(level = "trace")]
pub async fn aur_search(query: &str, options: Options) {
    let verbosity = options.verbosity;
    let packages = rpcsearch(query.to_string())
        .await
        .silent_unwrap(AppExitCode::RpcError);
    let total_results = packages.len();

    for package in &packages {
        println!(
            "aur/{} {}\n    {}",
            package.name, package.version, package.description
        )
    }

    if verbosity >= 1 {
        log!("Found {total_results} resuls for \"{query}\" in AUR",);
    }
}

#[tracing::instrument(level = "trace")]
pub async fn repo_search(query: &str, options: Options) {
    let verbosity = options.verbosity;
    let output = ShellCommand::pacman()
        .arg("-Ss")
        .arg(query)
        .wait_with_output()
        .await
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
