use std::process::exit;

use crate::internal::exit_code::AppExitCode;
use crate::logging::get_logger;
use crate::logging::handler::PromptDefault;

#[macro_export]
/// Macro for printing a message and destructively exiting
macro_rules! crash {
    ($exit_code:expr, $($arg:tt)+) => {
        $crate::internal::utils::log_and_crash(format!($($arg)+), $exit_code)
    }
}

#[macro_export]
/// Macro for prompting the user with a yes/no question.
macro_rules! prompt {
    (default yes, $($arg:tt)+) => {
        $crate::internal::utils::prompt_yn(format!($($arg)+), $crate::logging::handler::PromptDefault::Yes)
    };
    (default no, $($arg:tt)+) => {
        $crate::internal::utils::prompt_yn(format!($($arg)+), $crate::logging::handler::PromptDefault::No)
    };
    (no default, $($arg:tt)+) => {
        $crate::internal::utils::prompt_yn(format!($($arg)+), $crate::logging::handler::PromptDefault::None)
    }
}

/// Logs a message and exits the program with the given exit code.
pub fn log_and_crash(msg: String, exit_code: AppExitCode) -> ! {
    tracing::error!(msg);
    exit(exit_code as i32);
}

/// Prompts the user for a yes/no answer.
pub fn prompt_yn(question: String, prompt_default: PromptDefault) -> bool {
    get_logger().prompt(question, prompt_default)
}
