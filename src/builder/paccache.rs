use crate::internal::{commands::ShellCommand, error::AppResult};

#[derive(Debug, Default)]
pub struct PaccacheBuilder {
    keep: i32,
    keep_ins: bool,
    quiet: bool,
}

impl PaccacheBuilder {
    pub fn keep(mut self, keep: i32) -> Self {
        self.keep = keep;

        self
    }

    pub fn keep_ins(mut self, keep_ins: bool) -> Self {
        self.keep_ins = keep_ins;

        self
    }

    pub fn quiet(mut self, quiet: bool) -> Self {
        self.quiet = quiet;

        self
    }

    pub async fn remove(self) -> AppResult<()> {
        let mut command = ShellCommand::paccache().elevated();

        if self.quiet {
            command = command.arg("-q");
        }

        if self.keep_ins {
            command = command.arg("-u")
        }

        command
            .args(&["-r", &format!("-k{}", self.keep.to_string())])
            .wait_success()
            .await
    }
}
