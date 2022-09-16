use crossterm::style::Stylize;

use crate::builder::pacdiff::PacdiffBuilder;
use crate::internal::config::Config;
use crate::logging::get_logger;
use crate::prompt;

use super::prompt_sudo_single;

/// Searches the filesystem for .pacnew files and helps the user deal with them.
#[tracing::instrument(level = "trace")]
pub async fn detect() {
    prompt_sudo_single().await.expect("Sudo prompt failed");
    let pb = get_logger().new_progress_spinner();
    pb.set_message("Scanning for pacnew files");

    let mut pacnew = vec![];

    // Run `find` to find pacnew files and split by lines into a vec
    let find = PacdiffBuilder::list().await.unwrap();
    let find_lines = find.stdout.split('\n');
    for line in find_lines {
        if !line.is_empty() {
            pacnew.push(line.to_string());
        }
    }

    // If pacnew files are found, warn the user and prompt to pacdiff
    if pacnew.is_empty() {
        pb.finish_with_message("No .pacnew files found".bold().to_string());
        get_logger().reset_output_type();
    } else {
        pb.finish_with_message("pacnew files found".bold().to_string());
        get_logger().reset_output_type();
        tracing::info!(
            "It appears that at least one program you have installed / upgraded has installed a .pacnew config file. \
            These are created when you have modified a program's configuration, and a package upgrade could not automatically merge the new file. \
            You can deal with those files by running {}.",
            "sudo pacdiff".reset().magenta()
        );

        let choice = prompt!(default no, "Would you like to run pacdiff now?");
        if choice {
            let config = Config::get();
            if config.base.pacdiff_warn {
                tracing::warn!("Pacdiff uses vimdiff by default to edit files for merging. You can focus panes by mousing over them and pressing left click, and scroll up and down using your mouse's scroll wheel (or the arrow keys). To exit vimdiff, press the following key combination: ESC, :qa!, ENTER");
                tracing::warn!("You can surpress this warning in the future by setting `pacdiff_warn` to \"false\" in ~/.config/ame/config.toml");

                if prompt!(default no, "Continue?") {
                    PacdiffBuilder::pacdiff().await.unwrap();
                }
            } else {
                PacdiffBuilder::pacdiff().await.unwrap();
            }
        }
    }
}
