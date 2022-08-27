#![allow(clippy::module_name_repetitions)]

use serde::Deserialize;
use std::{env, fs};

use crate::{crash, AppExitCode};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub base: ConfigBase,
    pub extra: ConfigExtra,
}

#[derive(Debug, Deserialize)]
pub struct ConfigBase {
    pub pacdiff_warn: bool,
    pub highlight_optdepends: bool,
    pub powerpill: bool,
}

#[derive(Debug, Deserialize)]
pub struct ConfigExtra {
    pub uwu: Option<bool>,
    pub uwu_debug: Option<bool>,
    pub review_user_shell: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base: ConfigBase {
                pacdiff_warn: true,
                highlight_optdepends: true,
                powerpill: false,
            },
            extra: ConfigExtra {
                uwu: None,
                uwu_debug: None,
                review_user_shell: false,
            },
        }
    }
}

pub fn read() -> Config {
    let file = fs::read_to_string(format!(
        "{}/{}",
        env::var("HOME").unwrap(),
        ".config/ame/config.toml"
    ))
    .unwrap_or_else(|e| {
        crash!(
            AppExitCode::ConfigParseError,
            "Couldn't find config file: {}",
            e
        );
    });
    toml::from_str(&file).unwrap_or_else(|e| {
        crash!(
            AppExitCode::ConfigParseError,
            "Could not parse config file: {}",
            e
        );
    })
}
