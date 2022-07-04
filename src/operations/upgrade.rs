use crate::internal::commands::ShellCommand;
use crate::internal::error::SilentUnwrap;
use crate::internal::exit_code::AppExitCode;
use crate::internal::rpc::rpcinfo;
use crate::operations::aur_install::aur_install;
use crate::{info, log, prompt, Options};

pub fn upgrade(options: Options) {
    let verbosity = options.verbosity;
    let noconfirm = options.noconfirm;

    let mut pacman_args = vec!["-Syu"];
    if noconfirm {
        pacman_args.push("--noconfirm");
    }

    if verbosity >= 1 {
        log("Upgrading repo packages".to_string());
    }

    let pacman_result = ShellCommand::pacman()
        .elevated()
        .args(pacman_args)
        .wait()
        .silent_unwrap(AppExitCode::PacmanError);

    if pacman_result.success() {
        info("Successfully upgraded repo packages".to_string());
    } else {
        let cont = prompt(
            "Failed to upgrade repo packages, continue to upgrading AUR packages?".to_string(),
            false,
        );
        if !cont {
            info("Exiting".to_string());
            std::process::exit(AppExitCode::PacmanError as i32);
        }
    }

    if verbosity >= 1 {
        log("Upgrading AUR packages".to_string());
    }

    let res = crate::database::query(options);

    if verbosity >= 1 {
        log(format!("{:?}", &res));
    }

    let mut aur_upgrades = vec![];
    for r in res {
        let re = r.clone();
        let ver = rpcinfo(r.name);
        if ver.package.unwrap().version != r.version {
            aur_upgrades.push(re.name);
        }
    }

    if !aur_upgrades.is_empty() {
        aur_install(aur_upgrades, options);
    } else {
        info("No upgrades available for installed AUR packages".to_string());
    }
}
