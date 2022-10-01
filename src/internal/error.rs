use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io;

use crate::internal::exit_code::AppExitCode;
use crate::{fl, fl_crash};

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum AppError {
    Io(std::io::Error),
    Other(String),
    Rpc(aur_rpc::error::RPCError),
    NonZeroExit,
    BuildStepViolation,
    BuildError { pkg_name: String },
    UserCancellation,
    MissingDependencies(Vec<String>),
    MakePkg(String),
    MinusError(minus::MinusError),
    FmtError(std::fmt::Error),
    AlpmError(crate::internal::alpm::Error),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Io(io) => Display::fmt(io, f),
            AppError::Rpc(e) => Display::fmt(e, f),
            AppError::Other(s) => Display::fmt(s, f),
            AppError::NonZeroExit => Display::fmt(&fl!("non-zero-exit"), f),
            AppError::BuildStepViolation => Display::fmt(&fl!("build-step-violation"), f),
            AppError::BuildError { pkg_name } => write!(f, "{} {pkg_name}", fl!("build-error")),
            AppError::UserCancellation => write!(f, "{}", fl!("user-cancel")),
            AppError::MissingDependencies(deps) => {
                write!(f, "{} {}", fl!("missing-deps"), deps.join(", "))
            }
            AppError::MakePkg(msg) => write!(f, "{} {msg}", fl!("makepkg-err")),
            AppError::MinusError(e) => Display::fmt(e, f),
            AppError::FmtError(e) => Display::fmt(e, f),
            AppError::AlpmError(e) => Display::fmt(e, f),
        }
    }
}

impl Error for AppError {}

impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<aur_rpc::error::RPCError> for AppError {
    fn from(e: aur_rpc::error::RPCError) -> Self {
        Self::Rpc(e)
    }
}

impl From<String> for AppError {
    fn from(string: String) -> Self {
        Self::Other(string)
    }
}

impl From<&str> for AppError {
    fn from(string: &str) -> Self {
        Self::from(string.to_string())
    }
}

impl From<minus::MinusError> for AppError {
    fn from(e: minus::MinusError) -> Self {
        Self::MinusError(e)
    }
}

impl From<std::fmt::Error> for AppError {
    fn from(e: std::fmt::Error) -> Self {
        Self::FmtError(e)
    }
}

impl From<crate::internal::alpm::Error> for AppError {
    fn from(e: crate::internal::alpm::Error) -> Self {
        Self::AlpmError(e)
    }
}

pub trait SilentUnwrap<T> {
    fn silent_unwrap(self, error_code: AppExitCode) -> T;
}

impl<T> SilentUnwrap<T> for AppResult<T> {
    fn silent_unwrap(self, exit_code: AppExitCode) -> T {
        match self {
            Ok(val) => val,
            Err(e) => {
                tracing::debug!("{e}");
                fl_crash!(exit_code, "error-occurred")
            }
        }
    }
}
