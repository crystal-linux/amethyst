use crate::internal::commands::ShellCommand;
use crate::internal::config;
use crate::internal::error::SilentUnwrap;
use crate::internal::exit_code::AppExitCode;
use crate::logging::get_logger;
use crate::prompt;

use super::prompt_sudo_single;

/// Searches the filesystem for .pacnew files and helps the user deal with them.
pub async fn detect() {
    prompt_sudo_single().await.expect("Sudo prompt failed");
    let pb = get_logger().new_progress_spinner();
    pb.set_message("Scanning for pacnew files");

    let mut pacnew = vec![];

    // Run `find` to find pacnew files and split by lines into a vec
    let find = ShellCommand::pacdiff()
        .args(&["-o", "-f"])
        .elevated()
        .wait_with_output()
        .await
        .silent_unwrap(AppExitCode::PacmanError);
    let find_lines = find.stdout.split('\n');
    for line in find_lines {
        if !line.is_empty() {
            pacnew.push(line.to_string());
        }
    }

    // If pacnew files are found, warn the user and prompt to pacdiff
    if pacnew.is_empty() {
        pb.finish_with_message("No pacnew files found");
        get_logger().reset_output_type();
    } else {
        pb.finish_with_message("It appears that at least one program you have installed / upgraded has installed a .pacnew config file. These are created when you have modified a program's configuration, and a package upgrade could not automatically merge the new file.");
        get_logger().reset_output_type();

        let choice = prompt!(default no, "Would you like Amethyst to run pacdiff to deal with this? You can always deal with this later by running `sudo pacdiff`");
        if choice {
            let config = config::read();
            if config.base.pacdiff_warn {
                ShellCommand::pacdiff()
                    .elevated()
                    .wait()
                    .await
                    .silent_unwrap(AppExitCode::PacmanError);
            } else {
                tracing::warn!("Pacdiff uses vimdiff by default to edit files for merging. You can focus panes by mousing over them and pressing left click, and scroll up and down using your mouse's scroll wheel (or the arrow keys). To exit vimdiff, press the following key combination: ESC, :qa!, ENTER");
                tracing::warn!("You can surpress this warning in the future by setting `pacdiff_warn` to \"false\" in ~/.config/ame/config.toml");
                let cont = prompt!(default no, "Continue?");
                if cont {
                    ShellCommand::pacdiff()
                        .elevated()
                        .wait()
                        .await
                        .silent_unwrap(AppExitCode::PacmanError);
                }
            }
        }
    }
}
