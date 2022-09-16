use std::path::{Path, PathBuf};

use crate::internal::{commands::ShellCommand, error::AppResult};

#[derive(Debug, Default)]
pub struct RmBuilder {
    recursive: bool,
    force: bool,
    directory: PathBuf,
}

impl RmBuilder {
    pub fn recursive(mut self, recursive: bool) -> Self {
        self.recursive = recursive;

        self
    }

    pub fn force(mut self, force: bool) -> Self {
        self.force = force;

        self
    }

    pub fn directory<P: AsRef<Path>>(mut self, directory: P) -> Self {
        self.directory = directory.as_ref().into();

        self
    }

    #[tracing::instrument(level = "trace")]
    pub async fn build(self) -> AppResult<()> {
        let mut command = ShellCommand::rm().elevated();

        if self.recursive {
            command = command.arg("-r");
        }

        if self.force {
            command = command.arg("-f");
        }

        command = command.arg(self.directory);

        command.wait_success().await
    }
}
