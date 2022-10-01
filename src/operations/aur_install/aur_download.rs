use aur_rpc::PackageInfo;

use futures::future;

use crate::{
    fl_info,
    internal::{dependencies::DependencyInformation, error::AppResult, structs::Options},
    multi_progress, normal_output,
    operations::BuildContext,
};

use super::aur_review::AurReview;

pub struct AurDownload {
    pub options: Options,
    pub package_infos: Vec<PackageInfo>,
    pub packages: Vec<String>,
    pub dependencies: Vec<DependencyInformation>,
}

impl AurDownload {
    #[tracing::instrument(level = "trace", skip_all)]
    pub async fn download_sources(self) -> AppResult<AurReview> {
        tracing::info!("Downloading sources");
        multi_progress!();

        let contexts = future::try_join_all(
            self.package_infos
                .into_iter()
                .map(BuildContext::from)
                .map(super::common::download_aur_source),
        )
        .await?;

        normal_output!();
        fl_info!("all-sources-ready");

        Ok(AurReview {
            options: self.options,
            packages: self.packages,
            dependencies: self.dependencies,
            contexts,
        })
    }
}
