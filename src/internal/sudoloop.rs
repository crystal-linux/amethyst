use crate::ShellCommand;
use std::thread;
use std::time::Duration;

/// Loop sudo so it doesn't time out
pub fn start_sudoloop() {
    prompt_sudo();
    std::thread::spawn(|| loop {
        prompt_sudo();
        thread::sleep(Duration::from_secs(3 * 60))
    });
}

fn prompt_sudo() {
    while ShellCommand::sudo().arg("-v").wait_success().is_err() {}
}
