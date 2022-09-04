use std::path::{Path, PathBuf};

use crate::{
    internal::{commands::ShellCommand, error::AppResult},
    with_suspended_output,
};

#[derive(Default)]
pub struct PagerBuilder {
    path: PathBuf,
}

impl PagerBuilder {
    pub fn path<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.path = path.as_ref().into();

        self
    }

    pub async fn open(self) -> AppResult<()> {
        with_suspended_output!({ ShellCommand::pager().arg(self.path).wait_success().await })
    }
}
