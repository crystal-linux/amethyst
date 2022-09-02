use std::collections::HashSet;

use aur_rpc::PackageInfo;
use futures::future;

use crate::builder::pacman::{PacmanQueryBuilder, PacmanSearchBuilder};

use super::error::AppResult;
use lazy_regex::regex;

#[derive(Clone, Debug)]
pub struct DependencyInformation {
    pub depends: DependencyCollection,
    pub make_depends: DependencyCollection,
}

#[derive(Clone, Debug, Default)]
pub struct DependencyCollection {
    pub aur: Vec<PackageInfo>,
    pub repo: Vec<String>,
    pub not_found: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Dependency {
    pub name: String,
    #[allow(unused)]
    pub condition: Option<Condition>,
    #[allow(unused)]
    pub version: Option<String>,
}

#[derive(Clone, Debug)]
pub enum Condition {
    Gt,
    Ge,
    Eq,
    Le,
    Lt,
}

impl Condition {
    pub fn try_from_str(s: &str) -> Option<Self> {
        match s {
            "=" => Some(Self::Eq),
            "<=" => Some(Self::Le),
            ">=" => Some(Self::Ge),
            ">" => Some(Self::Gt),
            "<" => Some(Self::Lt),
            _ => None,
        }
    }
}

impl DependencyInformation {
    /// Resolves all dependency information for a given package
    #[tracing::instrument(level = "trace")]
    pub async fn for_package(package: &PackageInfo) -> AppResult<Self> {
        let make_depends = Self::resolve_make_depends(package).await?;
        let depends = Self::resolve_depends(package).await?;

        Ok(Self {
            depends,
            make_depends,
        })
    }

    /// Resolves all make dependencies for a package
    #[tracing::instrument(level = "trace")]
    async fn resolve_make_depends(package: &PackageInfo) -> AppResult<DependencyCollection> {
        let mut packages_to_resolve: HashSet<String> = package
            .make_depends
            .iter()
            .filter_map(|d| Self::map_dep_to_name(d))
            .collect();

        Self::filter_fulfilled_dependencies(&mut packages_to_resolve).await?;
        let mut already_searched = HashSet::new();
        let mut dependencies = DependencyCollection::default();

        while !packages_to_resolve.is_empty() {
            already_searched.extend(packages_to_resolve.iter().cloned());
            Self::extend_by_repo_packages(&mut packages_to_resolve, &mut dependencies).await?;

            let mut aur_packages = aur_rpc::info(&packages_to_resolve).await?;
            aur_packages.iter().for_each(|p| {
                packages_to_resolve.remove(&p.metadata.name);
            });
            let not_found = std::mem::take(&mut packages_to_resolve);

            dependencies
                .not_found
                .append(&mut not_found.into_iter().collect());

            packages_to_resolve = Self::get_filtered_make_depends(&aur_packages, &already_searched);
            Self::filter_fulfilled_dependencies(&mut packages_to_resolve).await?;
            dependencies.aur.append(&mut aur_packages);
        }

        Ok(dependencies)
    }

    /// Resolves all dependencies for a package
    #[tracing::instrument(level = "trace")]
    async fn resolve_depends(package: &PackageInfo) -> AppResult<DependencyCollection> {
        let mut packages_to_resolve: HashSet<String> = package
            .depends
            .iter()
            .filter_map(|d| Self::map_dep_to_name(d))
            .collect();
        Self::filter_fulfilled_dependencies(&mut packages_to_resolve).await?;
        let mut already_searched = HashSet::new();
        let mut dependencies = DependencyCollection::default();

        while !packages_to_resolve.is_empty() {
            already_searched.extend(packages_to_resolve.iter().cloned());
            Self::extend_by_repo_packages(&mut packages_to_resolve, &mut dependencies).await?;

            let mut aur_packages = aur_rpc::info(&packages_to_resolve).await?;
            aur_packages.iter().for_each(|p| {
                packages_to_resolve.remove(&p.metadata.name);
            });
            let not_found = std::mem::take(&mut packages_to_resolve);

            dependencies
                .not_found
                .append(&mut not_found.into_iter().collect());

            packages_to_resolve = Self::get_filtered_depends(&aur_packages, &already_searched);
            Self::filter_fulfilled_dependencies(&mut packages_to_resolve).await?;
            dependencies.aur.append(&mut aur_packages);
        }

        Ok(dependencies)
    }

    async fn extend_by_repo_packages(
        to_resolve: &mut HashSet<String>,
        dependencies: &mut DependencyCollection,
    ) -> AppResult<()> {
        let repo_deps = Self::find_repo_packages(to_resolve.clone()).await?;
        to_resolve.retain(|p| !repo_deps.contains(p));
        dependencies
            .repo
            .append(&mut repo_deps.into_iter().collect());

        Ok(())
    }

    fn get_filtered_make_depends(
        aur_packages: &[PackageInfo],
        searched: &HashSet<String>,
    ) -> HashSet<String> {
        aur_packages
            .iter()
            .flat_map(|p| {
                p.make_depends
                    .iter()
                    .filter_map(|d| Self::map_dep_to_name(d))
            })
            .filter(|d| !searched.contains(d))
            .collect()
    }

    fn get_filtered_depends(
        aur_packages: &[PackageInfo],
        searched: &HashSet<String>,
    ) -> HashSet<String> {
        aur_packages
            .iter()
            .flat_map(|p| p.depends.iter().filter_map(|d| Self::map_dep_to_name(d)))
            .filter(|d| !searched.contains(d))
            .collect()
    }

    async fn filter_fulfilled_dependencies(deps: &mut HashSet<String>) -> AppResult<()> {
        let mut fulfilled = HashSet::new();

        for dep in deps.iter() {
            if get_dependency_fulfilled(dep.clone()).await? {
                fulfilled.insert(dep.clone());
            }
        }

        deps.retain(|pkg| !fulfilled.contains(pkg));

        Ok(())
    }

    fn map_dep_to_name(dep: &str) -> Option<String> {
        Dependency::try_from_str(dep).map(|d| d.name)
    }

    #[tracing::instrument(level = "trace")]
    async fn find_repo_packages(pkg_names: HashSet<String>) -> AppResult<HashSet<String>> {
        let repo_searches = pkg_names.iter().cloned().map(|p| async {
            let search_result = PacmanSearchBuilder::default().query(&p).search().await?;
            AppResult::Ok((p, search_result))
        });
        let repo_deps = future::try_join_all(repo_searches).await?;
        let repo_deps: HashSet<String> = repo_deps
            .into_iter()
            .filter_map(|(p, found)| if found { Some(p) } else { None })
            .collect();

        Ok(repo_deps)
    }
}

impl Dependency {
    #[tracing::instrument(level = "trace")]
    pub fn try_from_str(s: &str) -> Option<Self> {
        let r =
            regex!(r#"^(?P<name>[\w\-]+)((?P<condition><=|=|>=|>|<)(?P<version>\d+(\.\d+)*))?$"#);
        let caps = r.captures(s)?;
        let name = caps["name"].to_string();
        let condition = caps
            .name("condition")
            .map(|c| c.as_str())
            .and_then(Condition::try_from_str);
        let version = caps.name("version").map(|v| v.as_str().into());
        tracing::debug!("Parsed dependency to {name} {condition:?} {version:?}");

        Some(Dependency {
            name,
            condition,
            version,
        })
    }
}

#[tracing::instrument(level = "trace")]
async fn get_dependency_fulfilled(name: String) -> AppResult<bool> {
    let not_found = PacmanQueryBuilder::all()
        .package(name)
        .query_with_output()
        .await?
        .is_empty();

    Ok(!not_found)
}
