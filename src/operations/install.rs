use crate::builder::pacman::PacmanInstallBuilder;
use crate::internal::exit_code::AppExitCode;
use crate::{crash, fl, Options};

#[tracing::instrument(level = "trace")]
pub async fn install(packages: Vec<String>, options: Options) {
    tracing::info!(
        "{}",
        fl!(
            "installing-packages-from-repos",
            packages = packages.join(", ")
        )
    );

    if !packages.is_empty() {
        tracing::debug!("Installing from repos: {:?}", &packages);

        let result = PacmanInstallBuilder::from_options(options)
            .packages(packages.clone())
            .install()
            .await;

        if result.is_err() {
            crash!(
                AppExitCode::PacmanError,
                "{}",
                fl!("error-install", error = packages.join(", "))
            );
        }

        tracing::debug!("Installing packages: {:?} was successful", &packages);
    }
}
