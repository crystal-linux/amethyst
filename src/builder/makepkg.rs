use std::fmt::Debug;
use std::path::{Path, PathBuf};

use tokio::process::Child;

use crate::internal::{
    commands::ShellCommand,
    error::{AppError, AppResult},
};

#[derive(Default, Debug, Clone)]
pub struct MakePkgBuilder {
    directory: PathBuf,
    clean: bool,
    no_deps: bool,
    install: bool,
    no_build: bool,
    no_confirm: bool,
    as_deps: bool,
    skip_pgp: bool,
    needed: bool,
    no_prepare: bool,
    force: bool,
}

impl MakePkgBuilder {
    /// Sets the working directory
    pub fn directory<D: AsRef<Path>>(mut self, dir: D) -> Self {
        self.directory = dir.as_ref().into();

        self
    }

    pub fn clean(mut self, clean: bool) -> Self {
        self.clean = clean;

        self
    }

    pub fn no_deps(mut self, no_deps: bool) -> Self {
        self.no_deps = no_deps;

        self
    }

    pub fn no_build(mut self, no_build: bool) -> Self {
        self.no_build = no_build;

        self
    }

    pub fn no_prepare(mut self, no_prepare: bool) -> Self {
        self.no_prepare = no_prepare;

        self
    }

    /// Mark packages as non-explicitly installed
    #[allow(clippy::wrong_self_convention)]
    pub fn as_deps(mut self, as_deps: bool) -> Self {
        self.as_deps = as_deps;

        self
    }

    /// Skip PGP signature checks
    pub fn skip_pgp(mut self, skip: bool) -> Self {
        self.skip_pgp = skip;

        self
    }

    /// Do not reinstall up to date packages
    pub fn needed(mut self, needed: bool) -> Self {
        self.needed = needed;

        self
    }

    pub fn force(mut self, force: bool) -> Self {
        self.force = force;

        self
    }

    pub async fn run(self) -> AppResult<()> {
        let output = self.build().wait_with_output().await?;

        if output.status.success() {
            Ok(())
        } else {
            Err(AppError::Other(output.stderr))
        }
    }

    pub fn spawn(self) -> AppResult<Child> {
        self.build().spawn(true)
    }

    /// Executes the makepkg command
    #[tracing::instrument(level = "trace")]
    fn build(self) -> ShellCommand {
        let mut command = ShellCommand::makepkg().working_dir(self.directory);

        if self.clean {
            command = command.arg("-c");
        }
        if self.no_deps {
            command = command.arg("-d")
        }
        if self.install {
            command = command.arg("-c");
        }
        if self.no_build {
            command = command.arg("-o");
        }
        if self.no_confirm {
            command = command.arg("--noconfirm")
        }
        if self.as_deps {
            command = command.arg("--asdeps")
        }
        if self.skip_pgp {
            command = command.arg("--skippgp")
        }
        if self.needed {
            command = command.arg("--needed");
        }
        if self.no_prepare {
            command = command.arg("--noprepare")
        }
        if self.force {
            command = command.arg("-f")
        }

        command
    }

    #[tracing::instrument(level = "trace")]
    pub async fn package_list<D: AsRef<Path> + Debug>(dir: D) -> AppResult<Vec<PathBuf>> {
        let result = ShellCommand::makepkg()
            .working_dir(dir.as_ref())
            .arg("--packagelist")
            .wait_with_output()
            .await?;

        if result.status.success() {
            let packages = result.stdout.lines().map(PathBuf::from).collect();

            Ok(packages)
        } else {
            Err(AppError::Other(result.stderr))
        }
    }
}
