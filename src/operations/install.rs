use crate::builder::pacman::PacmanInstallBuilder;
use crate::internal::exit_code::AppExitCode;
use crate::{fl_crash, fl_info, Options};

#[tracing::instrument(level = "trace")]
pub async fn install(packages: Vec<String>, options: Options) {
    fl_info!(
        "installing-packages-from-repos",
        packages = packages.join(", ")
    );

    if !packages.is_empty() {
        tracing::debug!("Installing from repos: {:?}", &packages);

        let result = PacmanInstallBuilder::from_options(options)
            .packages(packages.clone())
            .install()
            .await;

        if result.is_err() {
            fl_crash!(
                AppExitCode::PacmanError,
                "error-install",
                error = packages.join(", ")
            );
        }

        tracing::debug!("Installing packages: {:?} was successful", &packages);
    }
}
