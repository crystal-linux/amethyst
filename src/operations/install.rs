use crate::builder::pacman::PacmanInstallBuilder;
use crate::internal::exit_code::AppExitCode;
use crate::{crash, Options};

#[tracing::instrument(level = "trace")]
pub async fn install(packages: Vec<String>, options: Options) {
    tracing::info!("Installing packages {} from repos", &packages.join(", "));

    if !packages.is_empty() {
        tracing::debug!("Installing from repos: {:?}", &packages);

        let result = PacmanInstallBuilder::from_options(options)
            .packages(packages.clone())
            .install()
            .await;

        if result.is_err() {
            crash!(
                AppExitCode::PacmanError,
                "An error occured while installing packages: {}, aborting",
                packages.join(", "),
            );
        }

        tracing::debug!("Installing packages: {:?} was successful", &packages);
    }
}
