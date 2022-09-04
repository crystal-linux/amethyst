use std::path::{Path, PathBuf};

use crate::internal::{
    commands::ShellCommand,
    error::{AppError, AppResult},
};

#[derive(Debug, Default)]
pub struct GitCloneBuilder {
    url: String,
    directory: PathBuf,
}

impl GitCloneBuilder {
    pub fn url<S: ToString>(mut self, url: S) -> Self {
        self.url = url.to_string();

        self
    }

    pub fn directory<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.directory = path.as_ref().into();

        self
    }

    pub async fn clone(self) -> AppResult<()> {
        let result = ShellCommand::git()
            .arg("clone")
            .arg(self.url)
            .arg(self.directory)
            .wait_with_output()
            .await?;

        if result.status.success() {
            Ok(())
        } else {
            Err(AppError::Other(result.stderr))
        }
    }
}

#[derive(Debug, Default)]
pub struct GitPullBuilder {
    directory: PathBuf,
}

impl GitPullBuilder {
    pub fn directory<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.directory = path.as_ref().into();

        self
    }

    pub async fn pull(self) -> AppResult<()> {
        let result = ShellCommand::git()
            .arg("-C")
            .arg(self.directory)
            .arg("pull")
            .wait_with_output()
            .await?;

        if result.status.success() {
            Ok(())
        } else {
            Err(AppError::Other(result.stderr))
        }
    }
}
