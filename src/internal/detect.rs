use crate::internal::commands::ShellCommand;
use crate::internal::error::SilentUnwrap;
use crate::internal::exit_code::AppExitCode;
use crate::internal::strings::prompt;

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
        let choice = prompt("It appears that at least one program you have installed / upgraded has installed a .pacnew/.pacsave config file. Would you like to run pacdiff to deal with this?".to_string(), true);
        if choice {
            ShellCommand::pacdiff()
                .elevated()
                .wait()
                .silent_unwrap(AppExitCode::PacmanError);
        }
    }
}
