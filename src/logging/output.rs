use aur_rpc::PackageInfo;
use console::Alignment;
use crossterm::style::Stylize;

use crate::internal::dependencies::DependencyInformation;

use super::get_logger;

pub fn print_dependency_list(dependencies: &[DependencyInformation]) -> bool {
    let (deps_repo, makedeps_repo, deps_aur, makedeps_aur) = dependencies
        .iter()
        .map(|d| {
            (
                d.depends.repo.clone(),
                d.make_depends.repo.clone(),
                d.depends.aur.clone(),
                d.make_depends.aur.clone(),
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

    let mut empty = true;
    if !deps_repo.is_empty() {
        get_logger().print_newline();
        tracing::info!("Repo dependencies");
        get_logger().print_list(&deps_repo, "  ");
        empty = false;
    }
    if !deps_aur.is_empty() {
        get_logger().print_newline();
        tracing::info!("AUR dependencies");
        print_aur_package_list(&deps_aur);
        empty = false;
    }

    if !makedeps_repo.is_empty() {
        get_logger().print_newline();
        tracing::info!("Repo make dependencies");
        get_logger().print_list(&makedeps_repo, "  ");
        empty = false;
    }

    if !makedeps_aur.is_empty() {
        get_logger().print_newline();
        tracing::info!("AUR make dependencies");
        print_aur_package_list(&makedeps_aur);
        empty = false;
    }

    empty
}

pub fn print_aur_package_list(packages: &[PackageInfo]) {
    get_logger().print_list(
        packages.iter().map(|pkg| {
            format!(
                "{} version {} ({} votes)",
                console::pad_str(&pkg.metadata.name, 30, Alignment::Left, Some("...")).bold(),
                pkg.metadata.version.clone().dim(),
                pkg.metadata.num_votes,
            )
        }),
        "\n  ",
    );
}
