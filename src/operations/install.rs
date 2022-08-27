use crate::builder::pacman::PacmanInstallBuilder;
use crate::internal::exit_code::AppExitCode;
use crate::{crash, info, log, Options};

#[tracing::instrument(level = "trace")]
pub async fn install(packages: Vec<String>, options: Options) {
    info!("Installing packages {} from repos", &packages.join(", "));
    let verbosity = options.verbosity;

    if !packages.is_empty() {
        if verbosity >= 1 {
            log!("Installing from repos: {:?}", &packages);
        }

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

        if verbosity >= 1 {
            log!("Installing packages: {:?} was successful", &packages);
        }
    }
}
