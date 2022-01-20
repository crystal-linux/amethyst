use crate::internal::rpc::rpcinfo;
use crate::operations::aur_install::aur_install;
use crate::{log, Options};
use runas::Command;

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

    let res = crate::database::query("\"%\"", options);

    let mut aur_upgrades = vec![];
    for r in res {
        let re = r.clone();
        let ver = rpcinfo(r.name);
        if ver.package.unwrap().version != r.version {
            aur_upgrades.push(re.name);
        }
    }

    aur_install(aur_upgrades, options);
}
