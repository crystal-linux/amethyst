use std::time::Duration;

use crate::ShellCommand;

/// Loop sudo so it doesn't time out
pub async fn start_sudoloop() {
    prompt_sudo().await;
    tokio::task::spawn(async move {
        loop {
            prompt_sudo().await;
            tokio::time::sleep(Duration::from_secs(3 * 60)).await;
        }
    });
}

async fn prompt_sudo() {
    while ShellCommand::sudo().arg("-v").wait_success().await.is_err() {}
}
