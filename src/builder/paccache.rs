use crate::internal::{commands::ShellCommand, error::AppResult};

#[derive(Debug, Default)]
pub struct PaccacheBuilder {
    keep: i32,
    keep_ins_pkgs: bool,
    quiet_output: bool,
}

impl PaccacheBuilder {
    pub fn set_keep(mut self, keep: i32) -> Self {
        self.keep = keep;

        self
    }

    pub fn keep_ins_pkgs(mut self, keep_ins_pkgs: bool) -> Self {
        self.keep_ins_pkgs = keep_ins_pkgs;

        self
    }

    pub fn quiet_output(mut self, quiet_output: bool) -> Self {
        self.quiet_output = quiet_output;

        self
    }

    #[tracing::instrument(level = "trace")]
    pub async fn remove(self) -> AppResult<()> {
        let mut command = ShellCommand::paccache().elevated();

        if self.quiet_output {
            command = command.arg("-q");
        }

        if self.keep_ins_pkgs {
            command = command.arg("-u")
        }

        command
            .args(&["-r", &format!("-k{}", self.keep.to_string())])
            .wait_success()
            .await
    }
}
