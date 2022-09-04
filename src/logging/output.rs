use std::collections::{HashMap, HashSet};

use aur_rpc::PackageInfo;
use console::Alignment;
use crossterm::style::Stylize;

use crate::{builder::pacman::PacmanQueryBuilder, internal::dependencies::DependencyInformation};

use super::get_logger;

pub async fn print_dependency_list(dependencies: &[DependencyInformation]) -> bool {
    let (mut deps_repo, mut makedeps_repo, deps_aur, makedeps_aur) = dependencies
        .iter()
        .map(|d| {
            (
                d.depends.repo.iter().collect(),
                d.make_depends.repo.iter().collect(),
                d.depends.aur.iter().collect(),
                d.make_depends.aur.iter().collect(),
            )
        })
        .fold(
            (Vec::new(), Vec::new(), Vec::new(), Vec::new()),
            |mut acc, mut deps| {
                acc.0.append(&mut deps.0);
                acc.1.append(&mut deps.1);
                acc.2.append(&mut deps.2);
                acc.3.append(&mut deps.3);

                acc
            },
        );
    deps_repo.dedup();
    makedeps_repo.dedup();

    let mut empty = true;
    if !deps_repo.is_empty() {
        tracing::info!("Repo dependencies");
        get_logger().print_list(&deps_repo, "  ", 2);
        empty = false;
        get_logger().print_newline();
    }
    if !deps_aur.is_empty() {
        tracing::info!("AUR dependencies");
        print_aur_package_list(&deps_aur).await;
        empty = false;
        get_logger().print_newline();
    }

    if !makedeps_repo.is_empty() {
        tracing::info!("Repo make dependencies");
        get_logger().print_list(&makedeps_repo, "  ", 2);
        empty = false;
        get_logger().print_newline();
    }

    if !makedeps_aur.is_empty() {
        tracing::info!("AUR make dependencies");
        print_aur_package_list(&makedeps_aur).await;
        empty = false;
        get_logger().print_newline();
    }

    empty
}

pub async fn print_aur_package_list(packages: &[&PackageInfo]) -> bool {
    let pkgs = packages
        .iter()
        .map(|p| p.metadata.name.clone())
        .collect::<HashSet<_>>();
    let installed = PacmanQueryBuilder::all()
        .query_with_output()
        .await
        .unwrap()
        .into_iter()
        .filter(|p| pkgs.contains(&p.name))
        .map(|p| (p.name.clone(), p))
        .collect::<HashMap<_, _>>();

    get_logger().print_list(
        packages.iter().map(|pkg| {
            format!(
                "{} version {} ({} votes) {}",
                console::pad_str(&pkg.metadata.name, 30, Alignment::Left, Some("...")).bold(),
                pkg.metadata.version.clone().dim(),
                pkg.metadata.num_votes,
                if installed.contains_key(&pkg.metadata.name) {
                    "(Installed)"
                } else {
                    ""
                }
                .bold()
                .magenta()
            )
        }),
        "\n",
        2,
    );

    !installed.is_empty()
}
