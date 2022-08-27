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
