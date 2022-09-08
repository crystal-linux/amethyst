use std::{
    fmt::{Display, Formatter},
    path::Path,
};

use alpm::Alpm;
use alpm_utils::alpm_with_conf;
use pacmanconf::Config;

#[derive(Debug)]
pub enum Error {
    Alpm(alpm::Error),
    Pacmanconf(pacmanconf::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Alpm(e) => Display::fmt(e, f),
            Self::Pacmanconf(e) => Display::fmt(e, f),
        }
    }
}

impl From<alpm::Error> for Error {
    fn from(err: alpm::Error) -> Self {
        Error::Alpm(err)
    }
}

impl From<pacmanconf::Error> for Error {
    fn from(err: pacmanconf::Error) -> Self {
        Error::Pacmanconf(err)
    }
}

#[tracing::instrument(level = "trace")]
pub fn get_handler() -> Result<Alpm, Error> {
    let config = Config::from_file(Path::new("/etc/pacman.conf"))?;
    let alpm = alpm_with_conf(&config)?;

    tracing::debug!("Initialized alpm handler");

    Ok(alpm)
}
