#![allow(clippy::module_name_repetitions)]

use serde::Deserialize;
use std::{env, fs, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub base: ConfigBase,
    pub extra: ConfigExtra,
    pub bin: ConfigBin,
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

#[derive(Debug, Deserialize)]
pub struct ConfigBin {
    pub sudo: Option<String>,
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
            bin: ConfigBin {
                sudo: Some("sudo".to_string()),
            },
        }
    }
}

pub fn read() -> Config {
    let config_path = PathBuf::from(env::var("HOME").unwrap()).join(".config/ame/config.toml");
    match fs::read_to_string(config_path) {
        Ok(contents) => toml::from_str(&contents).expect("Could not parse the config file"),
        Err(_) => Config::default(),
    }
}
