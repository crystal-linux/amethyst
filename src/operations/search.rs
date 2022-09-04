use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;

use crate::internal::alpm::get_handler;
use crate::internal::error::SilentUnwrap;
use crate::internal::exit_code::AppExitCode;
use crate::internal::rpc::rpcsearch;
use crate::internal::utils::wrap_text;
use crate::logging::Printable;
use crate::Options;

use aur_rpc::SearchField;
use chrono::Local;
use chrono::TimeZone;
use colored::Colorize;

#[derive(Debug)]
pub struct PackageSearchResult {
    pub repo: String,
    pub name: String,
    pub version: String,
    pub group: Option<String>,
    pub out_of_date: Option<u64>,
    pub description: String,
}

impl Display for PackageSearchResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let repo = &self.repo;
        let name = &self.name;
        let version = &self.version;
        let group = self.group.clone().unwrap_or_default();
        let out_of_date = self.out_of_date.unwrap_or(0);
        let description = &self.description;

        format!("{repo}/{name} {version} {group} {out_of_date}\n    {description}").fmt(f)
    }
}

impl Printable for PackageSearchResult {
    fn to_print_string(&self) -> String {
        let repo = if &self.repo == "aur" {
            (self.repo.clone() + "/").bold().cyan()
        } else {
            (self.repo.clone() + "/").bold().purple()
        };
        let name = &self.name.bold();
        let version = &self.version.bold().green();
        let group = if self.group.is_some() {
            format!("({})", self.group.clone().unwrap()).bold().blue()
        } else {
            "".bold()
        };
        let out_of_date = if self.out_of_date.is_some() {
            format!(
                "[out of date: since {}]",
                Local
                    .timestamp(self.out_of_date.unwrap().try_into().unwrap(), 0)
                    .date_naive()
            )
            .bold()
            .red()
        } else {
            "".bold()
        };
        let description = wrap_text(&self.description, 4).join("\n");

        format!("{repo}{name} {version} {group} {out_of_date}\n    {description}")
    }
}

#[tracing::instrument(level = "trace")]
pub async fn aur_search(
    query: &str,
    by_field: Option<SearchBy>,
    options: Options,
) -> Vec<PackageSearchResult> {
    let packages = rpcsearch(query.to_string(), by_field.map(SearchBy::into))
        .await
        .silent_unwrap(AppExitCode::RpcError);
    let total_results = packages.len();

    tracing::debug!("Found {total_results} resuls for \"{query}\" in AUR",);

    let results: Vec<PackageSearchResult> = packages
        .into_iter()
        .map(|package| {
            let name = package.name;
            let version = package.version;
            let group = None;
            let out_of_date = package.out_of_date;
            let description = package.description;

            PackageSearchResult {
                repo: "aur".to_string(),
                name,
                version,
                group,
                out_of_date,
                description: description.unwrap_or_else(|| "No description".to_string()),
            }
        })
        .collect();

    results
}

#[tracing::instrument(level = "trace")]
pub async fn repo_search(query: &str, options: Options) -> Vec<PackageSearchResult> {
    let alpm = get_handler().unwrap();
    let dbs = alpm.syncdbs();

    let mut results = Vec::new();
    for db in dbs {
        let packages = db.search(vec![query].iter()).unwrap();

        for package in packages {
            let name = package.name();
            let version = package.version();
            let description = package.desc();
            let group = package.groups().first().map(|s| s.to_string());
            let out_of_date = None;

            let result = PackageSearchResult {
                repo: db.name().to_string(),
                name: name.to_string(),
                version: version.to_string(),
                group,
                out_of_date,
                description: description.unwrap_or("No description").to_string(),
            };

            results.push(result);
        }
    }

    tracing::debug!(
        "Found {} results for \"{}\" in repos",
        &results.len(),
        &query
    );

    results
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
