use std::str::FromStr;

use crate::internal::commands::ShellCommand;
use crate::internal::error::SilentUnwrap;
use crate::internal::exit_code::AppExitCode;
use crate::internal::rpc::rpcsearch;
use crate::Options;
use aur_rpc::SearchField;

#[tracing::instrument(level = "trace")]
pub async fn aur_search(query: &str, by_field: Option<SearchBy>, options: Options) {
    let packages = rpcsearch(query.to_string(), by_field.map(SearchBy::into))
        .await
        .silent_unwrap(AppExitCode::RpcError);
    let total_results = packages.len();

    for package in &packages {
        println!(
            "aur/{} {}\n    {}",
            package.name, package.version, package.description
        )
    }

    tracing::debug!("Found {total_results} resuls for \"{query}\" in AUR",);
}

#[tracing::instrument(level = "trace")]
pub async fn repo_search(query: &str, options: Options) {
    let output = ShellCommand::pacman()
        .arg("-Ss")
        .arg(query)
        .wait_with_output()
        .await
        .silent_unwrap(AppExitCode::PacmanError)
        .stdout;

    tracing::debug!(
        "Found {} results for \"{}\" in repos",
        &output.split('\n').count() / 2,
        &query
    );

    println!("{}", output)
}

/// Represents a field to search by
#[derive(Debug, Clone, Copy)]
pub enum SearchBy {
    /// Searches by name
    Name,
    /// Searches name and description
    NameDesc,
    /// Searches by package maintainer
    Maintainer,
    /// Searches for packages that depend on the given keywods
    Depends,
    /// Searches for packages that require the given keywords to be build
    MakeDepends,
    /// Searches for packages that optionally depend on the given keywods
    OptDepends,
    /// Searches for packages that require the given keywods to be present
    CheckDepends,
}

impl FromStr for SearchBy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let arg = match s {
            "name" => Self::Name,
            "name-desc" => Self::NameDesc,
            "maintainer" => Self::Maintainer,
            "depends" => Self::Depends,
            "makedepends" | "make-depends" => Self::MakeDepends,
            "optdepends" | "opt-depends" => Self::OptDepends,
            "checkdepends" | "check-depends" => Self::CheckDepends,
            directive => return Err(format!("Invalid search by directive '{directive}'")),
        };

        Ok(arg)
    }
}

#[allow(clippy::from_over_into)]
impl Into<SearchField> for SearchBy {
    fn into(self) -> SearchField {
        match self {
            SearchBy::Name => SearchField::Name,
            SearchBy::NameDesc => SearchField::NameDesc,
            SearchBy::Maintainer => SearchField::Maintainer,
            SearchBy::Depends => SearchField::Depends,
            SearchBy::MakeDepends => SearchField::MakeDepends,
            SearchBy::OptDepends => SearchField::OptDepends,
            SearchBy::CheckDepends => SearchField::CheckDepends,
        }
    }
}
