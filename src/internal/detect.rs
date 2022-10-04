use crossterm::style::Stylize;

use crate::builder::pacdiff::PacdiffBuilder;
use crate::internal::config::Config;
use crate::logging::get_logger;
use crate::{fl, fl_prompt, fl_warn, prompt};

/// Searches the filesystem for .pacnew files and helps the user deal with them.
#[tracing::instrument(level = "trace")]
pub async fn detect() {
    let pb = get_logger().new_progress_spinner();
    pb.set_message(fl!("scanning-pacnew-files"));

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
        pb.finish_with_message(fl!("no-pacnew-found").bold().to_string());
        get_logger().reset_output_type();
    } else {
        pb.finish_with_message(fl!("pacnew-found").bold().to_string());
        get_logger().reset_output_type();
        tracing::info!(
            "{} {}.",
            fl!("pacnew-warning"),
            "sudo pacdiff".reset().magenta()
        );

        let choice = fl_prompt!(default no, "run-pacdiff-now");
        if choice {
            let config = Config::get();
            if config.base.pacdiff_warn {
                fl_warn!("pacdiff-warning");

                if fl_prompt!(default no, "continue") {
                    PacdiffBuilder::pacdiff().await.unwrap();
                }
            } else {
                PacdiffBuilder::pacdiff().await.unwrap();
            }
        }
    }
}
