use crate::internal::commands::ShellCommand;
use crate::internal::error::SilentUnwrap;
use crate::internal::exit_code::AppExitCode;
use crate::{info, prompt, warn};

pub fn detect() {
    info!("Scanning for pacnew files");

    let mut pacnew = vec![];

    // Run `find` to find pacnew files and split by lines into a vec
    let find = std::process::Command::new("sudo")
        .arg("pacdiff")
        .arg("-f")
        .output()
        .unwrap();
    let find_lines = std::str::from_utf8(&find.stdout).unwrap().split('\n');
    for line in find_lines {
        if !line.is_empty() {
            pacnew.push(line.to_string());
        }
    }

    // If pacnew files are found, warn the user and prompt to pacdiff
    if !pacnew.is_empty() {
        let choice = prompt!(default false, "It appears that at least one program you have installed / upgraded has installed a .pacnew/.pacsave config file. Would you like to run pacdiff to deal with this? You can always deal with this later by running `sudo pacdiff`");
        if choice {
            warn!("Unless you've set an alternative using the DIFFPROG environment variable, pacdiff uses `vimdiff` by default to edit files for merging. Make sure you know how to exit vim before proceeding");
            let cont = prompt!(default false, "Continue?");
            if cont {
                ShellCommand::pacdiff()
                    .elevated()
                    .wait()
                    .silent_unwrap(AppExitCode::PacmanError);
            }
        }
    }
}
