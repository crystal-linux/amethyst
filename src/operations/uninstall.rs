use crate::internal::commands::ShellCommand;
use crate::internal::error::SilentUnwrap;
use crate::internal::exit_code::AppExitCode;
use crate::{log, Options};

/// Helps the user in uninstalling installed packages.
pub fn uninstall(packages: &[String], options: Options) {
    // Build pacman args
    let mut pacman_args = vec!["-Rs"];
    pacman_args.append(&mut packages.iter().map(String::as_str).collect());
    if options.noconfirm {
        pacman_args.push("--noconfirm");
    }
    let verbosity = options.verbosity;
    if verbosity >= 1 {
        log!("Uninstalling: {:?}", &packages);
    }

    // Uninstall packages
    ShellCommand::pacman()
        .elevated()
        .args(pacman_args)
        .wait_success()
        .silent_unwrap(AppExitCode::PacmanError);

    if verbosity >= 1 {
        log!("Uninstalling packages: {:?} exited with code 0", &packages);
    }
}
