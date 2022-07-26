use crate::internal::commands::ShellCommand;
use crate::internal::error::SilentUnwrap;
use crate::internal::exit_code::AppExitCode;
use crate::{crash, info, log, Options};

pub fn install(packages: Vec<String>, options: Options) {
    info!("Installing packages {} from repos", &packages.join(", "));

    // Build pacman args
    let mut opers = vec!["-S", "--needed"];
    if options.noconfirm {
        opers.push("--noconfirm");
    }
    if options.asdeps {
        opers.push("--asdeps");
    }
    let verbosity = options.verbosity;

    if !packages.is_empty() {
        if verbosity >= 1 {
            log!("Installing from repos: {:?}", &packages);
        }

        // Install packages
        let status = ShellCommand::pacman()
            .elevated()
            .args(opers)
            .args(&packages)
            .wait()
            .silent_unwrap(AppExitCode::PacmanError);
        if !status.success() {
            // If pacman failed, crash
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
