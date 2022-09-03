use std::path::{Path, PathBuf};

use crate::internal::{commands::ShellCommand, error::AppResult, structs::Options};

#[derive(Debug, Default)]
pub struct PacmanInstallBuilder {
    packages: Vec<String>,
    files: Vec<PathBuf>,
    as_deps: bool,
    no_confirm: bool,
    needed: bool,
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

    pub fn files<I: IntoIterator<Item = T>, T: AsRef<Path>>(mut self, files: I) -> Self {
        let mut files = files.into_iter().map(|f| f.as_ref().into()).collect();
        self.files.append(&mut files);

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

    pub fn needed(mut self, needed: bool) -> Self {
        self.needed = needed;

        self
    }

    #[tracing::instrument(level = "debug")]
    pub async fn install(self) -> AppResult<()> {
        let mut command = ShellCommand::pacman().elevated();

        if !self.packages.is_empty() {
            command = command.arg("-S");
        } else if !self.files.is_empty() {
            command = command.arg("-U");
        }

        if self.no_confirm {
            command = command.arg("--noconfirm")
        }

        if self.as_deps {
            command = command.arg("--asdeps")
        }
        if self.needed {
            command = command.arg("--needed")
        }

        command
            .args(self.packages)
            .args(self.files)
            .wait_success()
            .await
    }
}

#[derive(Debug)]
pub struct PacmanQueryBuilder {
    query_type: PacmanQueryType,
    color: PacmanColor,
    packages: Vec<String>,
}

#[derive(Debug)]
enum PacmanQueryType {
    Foreign,
    All,
    Info,
    Native,
}

#[derive(Clone, Copy, Debug)]
pub enum PacmanColor {
    #[allow(dead_code)]
    Always,
    Auto,
    Never,
}

impl Default for PacmanColor {
    fn default() -> Self {
        Self::Auto
    }
}

impl PacmanQueryBuilder {
    fn new(query_type: PacmanQueryType) -> Self {
        Self {
            query_type,
            color: PacmanColor::default(),
            packages: Vec::new(),
        }
    }

    pub fn all() -> Self {
        Self::new(PacmanQueryType::All)
    }

    pub fn foreign() -> Self {
        Self::new(PacmanQueryType::Foreign)
    }

    pub fn native() -> Self {
        Self::new(PacmanQueryType::Native)
    }

    pub fn info() -> Self {
        Self::new(PacmanQueryType::Info)
    }

    pub fn package(mut self, package: String) -> Self {
        self.packages.push(package);

        self
    }

    #[allow(dead_code)]
    pub fn packages<I: IntoIterator<Item = String>>(mut self, packages: I) -> Self {
        let mut packages = packages.into_iter().collect::<Vec<String>>();
        self.packages.append(&mut packages);

        self
    }

    pub fn color(mut self, color: PacmanColor) -> Self {
        self.color = color;

        self
    }

    #[tracing::instrument(level = "trace")]
    pub async fn query(self) -> AppResult<()> {
        self.build_command().wait_success().await
    }

    #[tracing::instrument(level = "trace")]
    pub async fn query_with_output(self) -> AppResult<Vec<BasicPackageInfo>> {
        let output = self.build_command().wait_with_output().await?;
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
        tracing::debug!("Query result: {packages:?}");

        Ok(packages)
    }

    fn build_command(self) -> ShellCommand {
        let mut command = ShellCommand::pacman().arg("-Q").arg("--color").arg("never");

        command = match self.query_type {
            PacmanQueryType::Foreign => command.arg("-m"),
            PacmanQueryType::Info => command.arg("-i"),
            PacmanQueryType::Native => command.arg("-n"),
            PacmanQueryType::All => command,
        };

        command = command.arg("--color");
        command = match self.color {
            PacmanColor::Always => command.arg("always"),
            PacmanColor::Auto => command.arg("auto"),
            PacmanColor::Never => command.arg("never"),
        };

        command.args(self.packages)
    }
}

#[derive(Clone, Debug)]
pub struct BasicPackageInfo {
    pub name: String,
    pub version: String,
}

#[derive(Default)]
pub struct PacmanSearchBuilder {
    query: String,
}

impl PacmanSearchBuilder {
    pub fn query<S: AsRef<str>>(mut self, query: S) -> Self {
        if !self.query.is_empty() {
            self.query.push(' ');
        }
        self.query.push_str(query.as_ref());

        self
    }

    /// Searches and returns if the execution result was ok
    pub async fn search(self) -> AppResult<bool> {
        let result = self.build_command().wait_with_output().await?;

        Ok(result.status.success())
    }

    fn build_command(self) -> ShellCommand {
        ShellCommand::pacman().arg("-Ss").arg(self.query)
    }
}
