use crossterm::style::Stylize;
use futures::future;

use crate::{
    internal::{
        dependencies::DependencyInformation,
        error::{AppError, AppResult},
        structs::Options,
    },
    logging::output::{print_aur_package_list, print_dependency_list},
    normal_output, prompt, spinner,
};

use super::aur_download::AurDownload;

pub struct AurFetch {
    pub options: Options,
    pub packages: Vec<String>,
}

impl AurFetch {
    #[tracing::instrument(level = "trace", skip_all)]
    pub async fn fetch_package_info(self) -> AppResult<AurDownload> {
        let pb = spinner!("Fetching package information");

        let package_infos = aur_rpc::info(&self.packages).await?;

        tracing::debug!("package info = {package_infos:?}");

        if package_infos.len() != self.packages.len() {
            pb.finish_with_message("Couldn't find all packages".red().to_string());
            let mut not_found = self.packages.clone();
            package_infos
                .iter()
                .for_each(|pkg| not_found.retain(|p| pkg.metadata.name != *p));
            return Err(AppError::MissingDependencies(not_found));
        }

        pb.finish_with_message("All packages found".green().to_string());
        normal_output!();

        print!("  "); // TODO: Make this less hacky
        if print_aur_package_list(&package_infos.iter().collect::<Vec<_>>()).await
            && !self.options.noconfirm
            && !prompt!(default yes, "Some packages are already installed. Continue anyway?")
        {
            return Err(AppError::UserCancellation);
        }

        let pb = spinner!("Fetching package information");

        let dependencies = future::try_join_all(
            package_infos
                .iter()
                .map(|pkg| async { DependencyInformation::for_package(pkg).await }),
        )
        .await?;

        pb.finish_and_clear();
        normal_output!();

        print_dependency_list(&dependencies).await;

        if !self.options.noconfirm
            && !prompt!(default yes, "Do you want to install these packages and package dependencies?")
        {
            Err(AppError::UserCancellation)
        } else {
            Ok(AurDownload {
                options: self.options,
                packages: self.packages,
                package_infos,
                dependencies,
            })
        }
    }
}
