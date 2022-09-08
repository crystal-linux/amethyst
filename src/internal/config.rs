#![allow(clippy::module_name_repetitions)]

use config::FileFormat;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs;

use super::utils::get_config_dir;

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Config {
    pub base: ConfigBase,
    pub extra: ConfigExtra,
    pub bin: ConfigBin,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigBase {
    pub pacdiff_warn: bool,
    pub highlight_optdepends: bool,
    pub powerpill: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigExtra {
    pub uwu: Option<bool>,
    pub uwu_debug: Option<bool>,
    pub review_user_shell: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigBin {
    pub sudo: Option<String>,
}

impl Default for ConfigBase {
    fn default() -> Self {
        Self {
            pacdiff_warn: true,
            highlight_optdepends: true,
            powerpill: false,
        }
    }
}

impl Default for ConfigExtra {
    fn default() -> Self {
        Self {
            uwu: None,
            uwu_debug: None,
            review_user_shell: false,
        }
    }
}

impl Default for ConfigBin {
    fn default() -> Self {
        Self {
            sudo: Some("sudo".to_string()),
        }
    }
}

impl Config {
    pub fn read() -> Self {
        let config_path = get_config_dir().join("config.toml");
        if config_path.exists() {
            let builder = config::Config::builder()
                .add_source(config::File::from_str(
                    &toml::ser::to_string(&Config::default()).unwrap(),
                    FileFormat::Toml,
                ))
                .add_source(config::File::with_name(config_path.to_str().unwrap()));
            let conf = builder.build().unwrap();
            conf.try_deserialize().unwrap()
        } else {
            let default_conf = Config::default();
            let toml_string = toml::ser::to_string_pretty(&default_conf).unwrap();
            fs::write(config_path, toml_string).unwrap();
            default_conf
        }
    }
    pub fn get() -> &'static Config {
        lazy_static! {
            static ref CONFIG: Config = Config::read();
        }
        &*CONFIG
    }
}
