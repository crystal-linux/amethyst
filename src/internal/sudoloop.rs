use std::time::Duration;

use crate::ShellCommand;

use super::error::AppResult;

/// Loop sudo so it doesn't time out
#[tracing::instrument(level = "trace")]
pub async fn start_sudoloop() {
    prompt_sudo().await;
    tokio::task::spawn(async move {
        loop {
            prompt_sudo().await;
            tokio::time::sleep(Duration::from_secs(3 * 60)).await;
        }
    });
}

#[tracing::instrument(level = "trace")]
async fn prompt_sudo() {
    while prompt_sudo_single().await.is_err() {}
}

#[tracing::instrument(level = "trace")]
pub async fn prompt_sudo_single() -> AppResult<()> {
    ShellCommand::sudo().arg("-v").wait_success().await
}
