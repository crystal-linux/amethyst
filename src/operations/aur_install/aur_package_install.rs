use crate::{
    builder::{makepkg::MakePkgBuilder, pacman::PacmanInstallBuilder},
    internal::{dependencies::DependencyInformation, error::AppResult, structs::Options},
    numeric,
    operations::aur_install::{
        common::build_and_install, make_dependency_removal::MakeDependencyRemoval,
    },
};

use super::BuildContext;

pub struct AurPackageInstall {
    pub options: Options,
    pub packages: Vec<String>,
    pub dependencies: Vec<DependencyInformation>,
    pub contexts: Vec<BuildContext>,
}

impl AurPackageInstall {
    #[tracing::instrument(level = "trace", skip_all)]
    pub async fn install_packages(self) -> AppResult<MakeDependencyRemoval> {
        tracing::info!(
            "Installing {}",
            numeric!(self.contexts.len(), "package"["s"])
        );
        build_and_install(
            self.contexts,
            MakePkgBuilder::default(),
            PacmanInstallBuilder::default().no_confirm(self.options.noconfirm),
        )
        .await?;

        Ok(MakeDependencyRemoval {
            options: self.options,
            dependencies: self.dependencies,
        })
    }
}
