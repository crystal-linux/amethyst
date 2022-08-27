use crate::internal::{commands::ShellCommand, error::AppResult, structs::Options};

pub struct PacmanWrapper;

impl PacmanWrapper {
    pub async fn install(args: PacmanInstallArgs) -> AppResult<()> {
        let mut command = ShellCommand::pacman().elevated().arg("-S").arg("--needed");

        if args.no_confirm {
            command = command.arg("--noconfirm")
        }

        if args.as_deps {
            command = command.arg("--asdeps")
        }

        command.args(args.packages).wait_success().await
    }
}

#[derive(Debug, Default)]
pub struct PacmanInstallArgs {
    packages: Vec<String>,
    as_deps: bool,
    no_confirm: bool,
}

impl PacmanInstallArgs {
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
}
