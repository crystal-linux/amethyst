use std::process::{Command, Stdio};

use crate::internal::{clean, rpc, structs};
use crate::{log, Options};

use super::error::SilentUnwrap;
use super::exit_code::AppExitCode;

#[tracing::instrument(level = "trace")]
pub async fn sort(input: &[String], options: Options) -> structs::Sorted {
    let mut repo_packages: Vec<String> = vec![];
    let mut aur_packages: Vec<String> = vec![];
    let mut missing_packages: Vec<String> = vec![];
    let verbosity = options.verbosity;

    let packages = clean(input, options);

    if verbosity >= 1 {
        log!("Sorting: {:?}", packages.join(" "));
    }

    for package in packages {
        let rs = Command::new("pacman")
            .arg("-Ss")
            .arg(format!("^{}$", &package))
            .stdout(Stdio::null())
            .status()
            .expect("Something has gone wrong");

        if let Some(0) = rs.code() {
            if verbosity >= 1 {
                log!("{} found in repos", package);
            }
            repo_packages.push(package.to_string());
        } else if rpc::rpcinfo(&package)
            .await
            .silent_unwrap(AppExitCode::RpcError)
            .is_some()
        {
            if verbosity >= 1 {
                log!("{} found in AUR", package);
            }
            aur_packages.push(package.to_string());
        } else {
            if verbosity >= 1 {
                log!("{} not found", package);
            }
            missing_packages.push(package.to_string());
        }
    }

    structs::Sorted::new(repo_packages, aur_packages, missing_packages)
}
