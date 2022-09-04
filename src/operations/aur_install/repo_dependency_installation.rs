use std::collections::HashSet;

use crate::{
    builder::pacman::PacmanInstallBuilder,
    internal::{dependencies::DependencyInformation, error::AppResult, structs::Options},
};

use super::{aur_dependency_installation::AurDependencyInstallation, BuildContext};

pub struct RepoDependencyInstallation {
    pub options: Options,
    pub packages: Vec<String>,
    pub dependencies: Vec<DependencyInformation>,
    pub contexts: Vec<BuildContext>,
}

impl RepoDependencyInstallation {
    #[tracing::instrument(level = "trace", skip_all)]
    pub async fn install_repo_dependencies(self) -> AppResult<AurDependencyInstallation> {
        let repo_dependencies: HashSet<&str> = self
            .dependencies
            .iter()
            .flat_map(DependencyInformation::all_repo_depends)
            .collect();

        if !repo_dependencies.is_empty() {
            tracing::info!("Installing repo dependencies");
            PacmanInstallBuilder::default()
                .as_deps(true)
                .packages(repo_dependencies)
                .no_confirm(self.options.noconfirm)
                .install()
                .await?;
        }
        Ok(AurDependencyInstallation {
            options: self.options,
            packages: self.packages,
            dependencies: self.dependencies,
            contexts: self.contexts,
        })
    }
}
