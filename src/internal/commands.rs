use std::env;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::process::{ExitStatus, Stdio};
use tokio::process::{Child, Command};

use crate::internal::config::Config;
use crate::internal::error::{AppError, AppResult};
use crate::internal::is_tty;

pub struct StringOutput {
    pub stdout: String,
    pub stderr: String,
    pub status: ExitStatus,
}

/// A wrapper around [std::process::Command] with predefined
/// commands used in this project as well as elevated access.
pub struct ShellCommand {
    command: String,
    args: Vec<OsString>,
    elevated: bool,
    working_dir: Option<PathBuf>,
}

impl ShellCommand {
    pub fn pacman() -> Self {
        Self::new("pacman")
    }

    pub fn paccache() -> Self {
        let paccache_cmd = Self::new("paccache");

        if is_tty() {
            paccache_cmd
        } else {
            paccache_cmd.arg("--nocolor")
        }
    }

    pub fn pacdiff() -> Self {
        Self::new("pacdiff")
    }

    pub fn makepkg() -> Self {
        Self::new("makepkg")
    }

    pub fn git() -> Self {
        Self::new("git")
    }

    #[allow(dead_code)]
    pub fn bash() -> Self {
        Self::new("bash")
    }

    pub fn sudo() -> Self {
        Self::new(Config::read().bin.sudo)
    }

    pub fn rm() -> Self {
        Self::new("rm")
    }

    pub fn pager() -> Self {
        let pager = env::var("PAGER").unwrap_or_else(|_| String::from("less"));

        Self::new(pager)
    }

    pub fn checkupdates() -> Self {
        Self::new("checkupdates")
    }

    fn new<S: ToString>(command: S) -> Self {
        Self {
            command: command.to_string(),
            args: Vec::new(),
            elevated: false,
            working_dir: None,
        }
    }

    /// Adds one argument
    pub fn arg<S: AsRef<OsStr>>(mut self, arg: S) -> Self {
        self.args.push(arg.as_ref().to_os_string());

        self
    }

    /// Adds a list of arguments
    pub fn args<I: IntoIterator<Item = S>, S: AsRef<OsStr>>(mut self, args: I) -> Self {
        self.args.append(
            &mut args
                .into_iter()
                .map(|a: S| a.as_ref().to_os_string())
                .collect(),
        );

        self
    }

    pub fn working_dir<D: AsRef<Path>>(mut self, dir: D) -> Self {
        self.working_dir = Some(dir.as_ref().into());

        self
    }

    /// Runs the command with sudo
    pub fn elevated(mut self) -> Self {
        self.elevated = true;

        self
    }

    /// Waits for the child to exit but returns an error when it exists with a non-zero status code
    pub async fn wait_success(self) -> AppResult<()> {
        let status = self.wait().await?;
        if status.success() {
            Ok(())
        } else {
            Err(AppError::NonZeroExit)
        }
    }

    /// Waits for the child to exit and returns the output status
    pub async fn wait(self) -> AppResult<ExitStatus> {
        let mut child = self.spawn(false)?;

        child.wait().await.map_err(AppError::from)
    }

    /// Waits with output until the program completed and
    /// returns the string output object
    pub async fn wait_with_output(self) -> AppResult<StringOutput> {
        let child = self.spawn(true)?;
        let output = child.wait_with_output().await?;
        let stdout = String::from_utf8(output.stdout).map_err(|e| AppError::from(e.to_string()))?;
        let stderr = String::from_utf8(output.stderr).map_err(|e| AppError::from(e.to_string()))?;

        Ok(StringOutput {
            status: output.status,
            stdout,
            stderr,
        })
    }

    pub fn spawn(self, piped: bool) -> AppResult<Child> {
        tracing::debug!("Running {} {:?}", self.command, self.args);

        let (stdout, stderr) = if piped {
            (Stdio::piped(), Stdio::piped())
        } else {
            (Stdio::inherit(), Stdio::inherit())
        };
        let mut command = if self.elevated {
            let mut cmd = Command::new(Config::read().bin.sudo);
            cmd.arg(self.command);

            cmd
        } else {
            Command::new(self.command)
        };
        if let Some(dir) = self.working_dir {
            command.current_dir(dir);
        }

        let child = command
            .args(self.args)
            .stdout(stdout)
            .stderr(stderr)
            .kill_on_drop(true)
            .spawn()?;

        Ok(child)
    }
}
