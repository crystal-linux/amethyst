use std::mem;

use tokio::{
    io::{AsyncRead, AsyncReadExt},
    process::{ChildStderr, ChildStdout},
};

use crate::internal::error::{AppError, AppResult};

pub struct StdioReader {
    stdout: ChildStdout,
    stderr: ChildStderr,
    stdout_line: Vec<u8>,
    stderr_line: Vec<u8>,
}

impl StdioReader {
    pub fn new(stdout: ChildStdout, stderr: ChildStderr) -> Self {
        Self {
            stdout,
            stderr,
            stdout_line: Vec::new(),
            stderr_line: Vec::new(),
        }
    }

    pub async fn read_line(&mut self) -> AppResult<String> {
        let line = tokio::select! {
            l = Self::read_stdio(&mut self.stdout, &mut self.stdout_line) => {l?}
            l = Self::read_stdio(&mut self.stderr, &mut self.stderr_line) => {l?}
        };

        Ok(line)
    }

    pub async fn read_stdio<R: AsyncRead + Unpin>(
        reader: &mut R,
        buf: &mut Vec<u8>,
    ) -> AppResult<String> {
        while let Ok(ch) = reader.read_u8().await {
            if ch == b'\n' {
                if !buf.is_empty() {
                    break;
                }
            } else {
                buf.push(ch);
            }
        }

        let line = mem::take(buf);
        if line.is_empty() {
            Err(AppError::from("stdio exhausted"))
        } else {
            Ok(String::from_utf8(line).unwrap())
        }
    }
}
