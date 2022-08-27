use crate::internal::{commands::ShellCommand, error::AppResult, structs::Options};

#[derive(Debug, Default)]
pub struct PacmanInstallBuilder {
    packages: Vec<String>,
    as_deps: bool,
    no_confirm: bool,
}

impl PacmanInstallBuilder {
    pub fn from_options(options: Options) -> Self {
        Self::default()
            .as_deps(options.asdeps)
            .no_confirm(options.noconfirm)
    }

    pub fn packages<I: IntoIterator<Item = String>>(mut self, packages: I) -> Self {
        let mut packages = packages.into_iter().collect();
        self.packages.append(&mut packages);

        self
    }

    pub fn no_confirm(mut self, no_confirm: bool) -> Self {
        self.no_confirm = no_confirm;

        self
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn as_deps(mut self, as_deps: bool) -> Self {
        self.as_deps = as_deps;

        self
    }

    #[tracing::instrument(level = "debug")]
    pub async fn install(self) -> AppResult<()> {
        let mut command = ShellCommand::pacman().elevated().arg("-S").arg("--needed");

        if self.no_confirm {
            command = command.arg("--noconfirm")
        }

        if self.as_deps {
            command = command.arg("--asdeps")
        }

        command.args(self.packages).wait_success().await
    }
}

#[derive(Debug, Default)]
pub struct PacmanQueryBuilder {
    foreign: bool,
}

impl PacmanQueryBuilder {
    /// Query for foreign packages
    pub fn foreign(mut self, foreign: bool) -> Self {
        self.foreign = foreign;

        self
    }

    #[tracing::instrument(level = "debug")]
    pub async fn query(self) -> AppResult<Vec<BasicPackageInfo>> {
        let mut command = ShellCommand::pacman().arg("-Q").arg("--color").arg("never");

        if self.foreign {
            command = command.arg("-m");
        }

        let output = command.wait_with_output().await?;
        let packages = output
            .stdout
            .split('\n')
            .filter(|p| !p.is_empty())
            .filter_map(|p| p.split_once(' '))
            .map(|(name, version)| BasicPackageInfo {
                name: name.to_string(),
                version: version.to_string(),
            })
            .collect();

        Ok(packages)
    }
}

#[derive(Clone, Debug)]
pub struct BasicPackageInfo {
    pub name: String,
    pub version: String,
}
