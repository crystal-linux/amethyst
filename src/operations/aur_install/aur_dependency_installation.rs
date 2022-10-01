use aur_rpc::PackageInfo;
use futures::future;

use crate::{
    builder::{makepkg::MakePkgBuilder, pacman::PacmanInstallBuilder},
    fl, fl_info,
    internal::{dependencies::DependencyInformation, error::AppResult},
    multi_progress, normal_output,
    operations::{
        aur_install::common::{build_and_install, create_dependency_batches, download_aur_source},
        BuildContext,
    },
};

use super::aur_package_install::AurPackageInstall;

pub struct AurDependencyInstallation {
    pub options: crate::internal::structs::Options,
    pub dependencies: Vec<DependencyInformation>,
    pub contexts: Vec<BuildContext>,
}

impl AurDependencyInstallation {
    #[tracing::instrument(level = "trace", skip_all)]
    pub async fn install_aur_dependencies(self) -> AppResult<AurPackageInstall> {
        let aur_dependencies: Vec<&PackageInfo> = self
            .dependencies
            .iter()
            .flat_map(DependencyInformation::all_aur_depends)
            .collect();

        if !aur_dependencies.is_empty() {
            fl_info!(
                "installing-from-aur",
                amountOfPkgs = format!(
                    "{} {}",
                    aur_dependencies.len(),
                    fl!("packages", pkgNum = aur_dependencies.len())
                )
            );
            let batches = create_dependency_batches(aur_dependencies);
            tracing::debug!("aur install batches: {batches:?}");

            for batch in batches {
                self.install(batch).await.unwrap();
            }
        }

        Ok(AurPackageInstall {
            options: self.options,
            dependencies: self.dependencies,
            contexts: self.contexts,
        })
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn install(&self, deps: Vec<&PackageInfo>) -> AppResult<()> {
        multi_progress!();

        let dep_contexts = future::try_join_all(
            deps.into_iter()
                .map(BuildContext::from)
                .map(download_aur_source),
        )
        .await?;

        normal_output!();

        build_and_install(
            dep_contexts,
            MakePkgBuilder::default().as_deps(true),
            PacmanInstallBuilder::default()
                .no_confirm(self.options.noconfirm)
                .as_deps(true),
        )
        .await?;

        Ok(())
    }
}
