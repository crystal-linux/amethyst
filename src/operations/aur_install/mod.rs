use aur_rpc::PackageInfo;

use std::path::{Path, PathBuf};

use crate::internal::error::{AppError, AppResult};

use crate::internal::exit_code::AppExitCode;
use crate::{cancelled, crash, fl, Options};

use self::aur_fetch::AurFetch;

mod aur_dependency_installation;
mod aur_download;
mod aur_fetch;
mod aur_package_install;
mod aur_review;
mod common;
mod make_dependency_removal;
mod repo_dependency_installation;

#[derive(Debug)]
pub struct BuildContext {
    pub package: PackageInfo,
    pub step: BuildStep,
}

#[derive(Debug)]
pub enum BuildStep {
    Download,
    Build(BuildPath),
    Install(PackageArchives),
    Done,
}

#[derive(Debug)]
pub struct BuildPath(pub PathBuf);

#[derive(Debug)]
pub struct PackageArchives(pub Vec<PathBuf>);

impl From<PackageInfo> for BuildContext {
    fn from(package: PackageInfo) -> Self {
        Self {
            package,
            step: BuildStep::Download,
        }
    }
}

impl From<&PackageInfo> for BuildContext {
    fn from(p: &PackageInfo) -> Self {
        Self::from(p.to_owned())
    }
}

impl BuildContext {
    pub fn build_path(&self) -> AppResult<&Path> {
        if let BuildStep::Build(path) = &self.step {
            Ok(&path.0)
        } else {
            Err(AppError::BuildStepViolation)
        }
    }

    pub fn packages(&self) -> AppResult<&Vec<PathBuf>> {
        if let BuildStep::Install(pkgs) = &self.step {
            Ok(&pkgs.0)
        } else {
            Err(AppError::BuildStepViolation)
        }
    }
}

pub struct AurInstall {
    options: Options,
    packages: Vec<String>,
}

impl AurInstall {
    pub fn new(options: Options, packages: Vec<String>) -> Self {
        Self { options, packages }
    }

    pub fn start(self) -> AurFetch {
        tracing::debug!("Installing from AUR: {:?}", &self.packages);
        AurFetch {
            options: self.options,
            packages: self.packages,
        }
    }
}

/// Installs a given list of packages from the aur
#[tracing::instrument(level = "trace")]
pub async fn aur_install(packages: Vec<String>, options: Options) {
    if let Err(e) = aur_install_internal(AurInstall::new(options, packages)).await {
        match e {
            AppError::Rpc(e) => {
                crash!(AppExitCode::RpcError, "{} {e}", fl!("aur-rpc-crash"))
            }
            AppError::BuildStepViolation => {
                crash!(AppExitCode::MakePkgError, "{}", fl!("failed-to-build"))
            }
            AppError::BuildError { pkg_name } => {
                crash!(
                    AppExitCode::MakePkgError,
                    "{} {pkg_name}",
                    fl!("failed-to-build")
                )
            }
            AppError::UserCancellation => {
                cancelled!();
            }
            AppError::MissingDependencies(deps) => {
                crash!(
                    AppExitCode::MissingDeps,
                    "{} {}",
                    fl!("missing-deps"),
                    deps.join(", ")
                )
            }
            AppError::MakePkg(msg) => {
                crash!(AppExitCode::MakePkgError, "{} {msg}", fl!("makepkg-failed"))
            }
            _ => crash!(AppExitCode::Other, "{}", fl!("unknown-error")),
        }
    }
}

async fn aur_install_internal(install: AurInstall) -> AppResult<()> {
    install
        .start()
        .fetch_package_info()
        .await?
        .download_sources()
        .await?
        .review_pkgbuild()
        .await?
        .install_repo_dependencies()
        .await?
        .install_aur_dependencies()
        .await?
        .install_packages()
        .await?
        .remove_make_deps()
        .await
}
