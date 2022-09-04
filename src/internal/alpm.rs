use std::path::Path;

use alpm::Alpm;
use alpm_utils::alpm_with_conf;
use pacmanconf::Config;

#[derive(Debug)]
pub enum Error {
    Alpm(alpm::Error),
    Pacmanconf(pacmanconf::Error),
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

pub fn get_handler() -> Result<Alpm, Error> {
    let config = Config::from_file(Path::new("/etc/pacman.conf"))?;
    let alpm = alpm_with_conf(&config)?;

    Ok(alpm)
}
