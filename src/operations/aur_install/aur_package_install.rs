use crate::{
    builder::{makepkg::MakePkgBuilder, pacman::PacmanInstallBuilder},
    fl,
    internal::{dependencies::DependencyInformation, error::AppResult, structs::Options},
    operations::aur_install::{
        common::build_and_install, make_dependency_removal::MakeDependencyRemoval,
    },
};

use super::BuildContext;

pub struct AurPackageInstall {
    pub options: Options,
    pub dependencies: Vec<DependencyInformation>,
    pub contexts: Vec<BuildContext>,
}

impl AurPackageInstall {
    #[tracing::instrument(level = "trace", skip_all)]
    pub async fn install_packages(self) -> AppResult<MakeDependencyRemoval> {
        tracing::info!(
            "Installing {} {}",
            self.contexts.len(),
            fl!("packages", pkgNum = self.contexts.len())
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
