use alpm::Alpm;
use lazy_static::lazy_static;
use pacmanconf::Config;

fn get_alpm() -> Alpm {
    alpm_utils::alpm_with_conf(get_pacman_config()).unwrap()
}

fn get_pacman_config() -> &'static Config {
    lazy_static! {
        static ref PACMAN_CONF: Config = Config::new().unwrap();
    }
    &PACMAN_CONF
}
