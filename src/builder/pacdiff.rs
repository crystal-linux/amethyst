use crate::internal::{
    commands::{ShellCommand, StringOutput},
    error::{AppError, AppResult},
};

#[derive(Debug, Default)]
pub struct PacdiffBuilder {}

impl PacdiffBuilder {
    pub async fn list() -> AppResult<StringOutput> {
        let result = ShellCommand::pacdiff()
            .args(&["-o", "-f"])
            .elevated()
            .wait_with_output()
            .await?;
        if result.status.success() {
            Ok(result)
        } else {
            Err(AppError::Other(result.stderr))
        }
    }

    pub async fn pacdiff() -> AppResult<()> {
        ShellCommand::pacdiff().elevated().wait_success().await
    }
}
