use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;

use crate::fl;
use crate::internal::alpm::Alpm;
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
use trigram::similarity;

#[derive(Debug)]
pub struct PackageSearchResult {
    pub repo: String,
    pub name: String,
    pub version: String,
    pub groups: Option<Vec<String>>,
    pub out_of_date: Option<u64>,
    pub installed: bool,
    pub description: Option<String>,
}

impl PackageSearchResult {
    pub fn score(&self, query: &str) -> f32 {
        similarity(query, &self.name)
    }
}

impl Display for PackageSearchResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let repo = &self.repo;
        let name = &self.name;
        let version = &self.version;
        let groups = if let Some(groups) = &self.groups {
            if groups.is_empty() {
                String::new()
            } else {
                format!("({})", groups.join(", "))
            }
        } else {
            String::new()
        };
        let out_of_date = if let Some(out_of_date) = self.out_of_date {
            format!(
                " [{} {}]",
                fl!("out-of-date"),
                Local
                    .timestamp(out_of_date.try_into().unwrap(), 0)
                    .date_naive()
            )
        } else {
            String::new()
        };
        let installed = if self.installed {
            format!(" [{}]", fl!("installed"))
        } else {
            String::new()
        };
        let description = wrap_text(
            self.description
                .clone()
                .unwrap_or_else(|| "No description".to_string()),
            4,
        )
        .join("\n");

        format!("{repo}{name} {version}{groups}{out_of_date}{installed}\n    {description}").fmt(f)
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
        let groups = if let Some(groups) = &self.groups {
            if groups.is_empty() {
                "".to_string()
            } else {
                format!(" ({})", groups.join(", "))
            }
        } else {
            "".to_string()
        }
        .bold()
        .blue();
        let out_of_date = if let Some(out_of_date) = self.out_of_date {
            format!(
                " [{} {}]",
                fl!("out-of-date"),
                Local
                    .timestamp(out_of_date.try_into().unwrap(), 0)
                    .date_naive()
            )
        } else {
            "".to_string()
        }
        .bold()
        .red();
        let installed = if self.installed {
            format!(" [{}]", fl!("installed"))
        } else {
            "".to_string()
        }
        .bold()
        .cyan();
        let description = wrap_text(
            self.description
                .clone()
                .unwrap_or_else(|| "No description".to_string()),
            4,
        )
        .join("\n");

        format!("{repo}{name} {version}{groups}{out_of_date}{installed}\n    {description}")
    }
}

#[tracing::instrument(level = "trace")]
pub async fn aur_search(
    query: &str,
    by_field: Option<SearchBy>,
    options: Options,
) -> Vec<PackageSearchResult> {
    let alpm = Alpm::new().unwrap();
    let alpm = alpm.handler();

    let local = alpm.localdb();
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
            let groups = None;
            let out_of_date = package.out_of_date;
            let installed = local.pkg(&*name).is_ok();
            let description = package.description;

            PackageSearchResult {
                repo: "aur".to_string(),
                name,
                version,
                groups,
                out_of_date,
                installed,
                description,
            }
        })
        .collect();

    results
}

#[tracing::instrument(level = "trace")]
pub async fn repo_search(query: &str, options: Options) -> Vec<PackageSearchResult> {
    let alpm = Alpm::new().unwrap();
    let alpm = alpm.handler();

    let local = alpm.localdb();
    let dbs = alpm.syncdbs();

    let mut results = Vec::new();
    for db in dbs {
        let packages = db.search(vec![query].iter()).unwrap();

        for package in packages {
            let name = package.name();
            let version = package.version();
            let groups = Some(
                package
                    .groups()
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect(),
            );
            let out_of_date = None;
            let installed = local.pkg(name).is_ok();
            let description = package.desc();

            let result = PackageSearchResult {
                repo: db.name().to_string(),
                name: name.to_string(),
                version: version.to_string(),
                groups,
                out_of_date,
                installed,
                description: Some(description.unwrap().to_string()),
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
