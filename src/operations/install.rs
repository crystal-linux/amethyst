use crate::internal::commands::ShellCommand;
use crate::internal::error::SilentUnwrap;
use crate::internal::exit_code::AppExitCode;
use crate::{crash, info, log, Options};

pub fn install(packages: Vec<String>, options: Options) {
    info(format!(
        "Installing packages {} from repos",
        &packages.join(", ")
    ));
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
            log(format!("Installing from repos: {:?}", &packages));
        }

        let status = ShellCommand::pacman()
            .elevated()
            .args(opers)
            .args(&packages)
            .wait()
            .silent_unwrap(AppExitCode::PacmanError);
        if !status.success() {
            crash(
                format!(
                    "An error occured while installing packages: {}, aborting",
                    packages.join(", ")
                ),
                AppExitCode::PacmanError,
            );
        }

        if verbosity >= 1 {
            log(format!(
                "Installing packages: {:?} was successful",
                &packages
            ));
        }
    }
}
