use runas::Command;

use crate::{info, log, Options};
use crate::internal::rpc::rpcinfo;
use crate::operations::aur_install::aur_install;

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

    Command::new("pacman")
        .args(&pacman_args)
        .status()
        .expect("Something has gone wrong");

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
