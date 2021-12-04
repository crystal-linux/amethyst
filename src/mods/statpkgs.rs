use crate::inf;
use crate::{
    err_rec, inssort, stat_add_pkg, stat_dump_dat, stat_get_value, stat_rem_pkg, uninstall,
};
use std::{env, fs};

pub fn rebuild(noconfirm: bool) {
    let file = format!("{}/.config/ame/pkgs.toml", env::var("HOME").unwrap());
    let database = fs::read_to_string(&file).expect("Can't Open Database");
    inf("installing crystal config".to_string());

    let file = format!("{}/.local/share/ame/aur_pkgs.db", env::var("HOME").unwrap());
    let connection = sqlite::open(file).unwrap();
    connection
        .execute(
            "
        CREATE TABLE IF NOT EXISTS static_pkgs (name TEXT, pin INTEGER);
        ",
        )
        .unwrap();

    let db_parsed = database.parse::<toml::Value>().expect("Invalid Database");
    let mut pkgs = Vec::new();
    if let Some(entry) = db_parsed.as_table() {
        for (key, value) in &*entry {
            let mut tempvec = Vec::new();
            // println!("{}", key);
            // println!("{}", format!("{}",value).replace("update = ", ""));
            tempvec.push(key.to_string());
            tempvec.push(format!("{}", value).replace("update = ", ""));
            pkgs.push(tempvec);
        }
    }
    let mut pkgs_to_add: Vec<Vec<String>> = Vec::new();
    let mut pkgs_to_install: Vec<String> = Vec::new();
    for i in pkgs {
        if !stat_get_value(&i[0], "name") {
            let tempvec = vec![i[0].to_string(), i[1].to_string()];
            pkgs_to_add.push(tempvec);
            pkgs_to_install.push(i[0].to_string());
        }
    }
    let mut config_no_change = 0;
    if !pkgs_to_install.is_empty() {
        inf(format!("Installing {}", pkgs_to_install.join(", ")));
        inssort(noconfirm, false, pkgs_to_install);
        for i in pkgs_to_add {
            stat_add_pkg(&i[1], &i[0]);
        }
        config_no_change += 1;
    }
    let dat_pkgs = stat_dump_dat();

    let mut pkgs = Vec::new();
    if let Some(entry) = db_parsed.as_table() {
        for (key, _value) in &*entry {
            pkgs.push(key);
        }
    }

    let mut pkgs_to_remove: Vec<String> = Vec::new();
    for i in dat_pkgs {
        if !pkgs.contains(&&i) {
            pkgs_to_remove.push(i.to_string());
        }
        config_no_change += 1;
    }
    if !pkgs_to_remove.is_empty() {
        inf(format!("Removing {}", pkgs_to_remove.join(", ")));
        stat_rem_pkg(&pkgs_to_remove);
        uninstall(noconfirm, pkgs_to_remove);
    }

    if config_no_change != 0 {
        inf("Rebuild Complete".to_string());
    } else {
        err_rec("Configuration not changed!".to_string());
    }
}
