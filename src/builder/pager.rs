use std::path::{Path, PathBuf};

use crate::{
    internal::{commands::ShellCommand, error::AppResult},
    logging::get_logger,
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
        get_logger().suspend();
        ShellCommand::pager().arg(self.path).wait_success().await?;
        get_logger().unsuspend();

        Ok(())
    }
}
