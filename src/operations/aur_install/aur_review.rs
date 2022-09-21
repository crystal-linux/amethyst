use tokio::fs;

use crate::{
    builder::pager::PagerBuilder,
    fl,
    internal::{
        dependencies::DependencyInformation,
        error::{AppError, AppResult},
        structs::Options,
        utils::get_cache_dir,
    },
    multi_select, prompt, select_opt,
};

use super::{repo_dependency_installation::RepoDependencyInstallation, BuildContext};

pub struct AurReview {
    pub options: Options,
    pub packages: Vec<String>,
    pub dependencies: Vec<DependencyInformation>,
    pub contexts: Vec<BuildContext>,
}

impl AurReview {
    #[tracing::instrument(level = "trace", skip_all)]
    pub async fn review_pkgbuild(self) -> AppResult<RepoDependencyInstallation> {
        if !self.options.noconfirm {
            let to_review = multi_select!(&self.packages, "{}", fl!("select-pkgs-review"));

            for pkg in to_review.into_iter().filter_map(|i| self.packages.get(i)) {
                self.review_single_package(pkg).await?;
            }
            if !prompt!(default yes, "{}", fl!("do-you-still-want-to-install")) {
                return Err(AppError::UserCancellation);
            }
        }
        Ok(RepoDependencyInstallation {
            options: self.options,
            dependencies: self.dependencies,
            contexts: self.contexts,
        })
    }

    async fn review_single_package(&self, pkg: &str) -> AppResult<()> {
        tracing::info!("{} {pkg}", fl!("reviewing"));
        let mut files_iter = fs::read_dir(get_cache_dir().join(pkg)).await?;
        let mut files = Vec::new();

        while let Some(file) = files_iter.next_entry().await? {
            let path = file.path();

            if path.is_file() {
                files.push(file.path());
            }
        }

        let file_names = files
            .iter()
            .map(|f| f.file_name().unwrap())
            .map(|f| f.to_string_lossy())
            .collect::<Vec<_>>();

        while let Some(selection) = select_opt!(&file_names, "{}", fl!("select-file-review")) {
            if let Some(path) = files.get(selection) {
                if let Err(e) = PagerBuilder::default().path(path).open().await {
                    tracing::debug!("Pager error {e}");
                }
            } else {
                break;
            }
        }

        tracing::info!("{}", fl!("done-reviewing-pkg", pkg = pkg));

        Ok(())
    }
}
