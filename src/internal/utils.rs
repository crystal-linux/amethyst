use std::fs;
use std::path::Path;
use std::process::exit;

use directories::ProjectDirs;
use textwrap::wrap;

use crate::internal::exit_code::AppExitCode;
use lazy_static::lazy_static;

use super::error::{AppError, SilentUnwrap};

#[macro_export]
/// Macro for printing a message and destructively exiting
macro_rules! crash {
    ($exit_code:expr, $($arg:tt)+) => {
        $crate::internal::utils::log_and_crash(format!($($arg)+), $exit_code)
    }
}

#[macro_export]
/// Cancelles the process
macro_rules! cancelled {
    () => {
        crash!(
            $crate::internal::exit_code::AppExitCode::UserCancellation,
            "Installation cancelled"
        )
    };
}

/// Logs a message and exits the program with the given exit code.
pub fn log_and_crash(msg: String, exit_code: AppExitCode) -> ! {
    tracing::error!(msg);
    exit(exit_code as i32);
}

pub fn get_cache_dir() -> &'static Path {
    lazy_static! {
        static ref CACHE_DIR: &'static Path = create_if_not_exist(get_directories().cache_dir());
    }

    *CACHE_DIR
}

fn get_directories() -> &'static ProjectDirs {
    lazy_static! {
        static ref DIRECTORIES: ProjectDirs = ProjectDirs::from("com", "crystal", "ame").unwrap();
    }

    &*DIRECTORIES
}

fn create_if_not_exist(dir: &Path) -> &Path {
    if !dir.exists() {
        fs::create_dir_all(dir)
            .map_err(AppError::from)
            .silent_unwrap(AppExitCode::FailedCreatingPaths)
    }

    dir
}

pub fn wrap_text<S: AsRef<str>>(s: S) -> Vec<String> {
    wrap(s.as_ref(), get_wrap_options())
        .into_iter()
        .map(String::from)
        .collect()
}

fn get_wrap_options() -> textwrap::Options<'static> {
    textwrap::Options::new(crossterm::terminal::size().unwrap().0 as usize - 2)
        .subsequent_indent("  ")
}
