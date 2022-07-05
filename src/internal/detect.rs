use crate::internal::commands::ShellCommand;
use crate::internal::error::SilentUnwrap;
use crate::internal::exit_code::AppExitCode;
use crate::{prompt, warn};

pub fn detect() {
    let mut pacnew = vec![];

    for entry in std::fs::read_dir("/etc").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.to_str().unwrap().contains(".pacnew") || path.to_str().unwrap().contains(".pacsave")
        {
            pacnew.push(path);
        }
    }

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
