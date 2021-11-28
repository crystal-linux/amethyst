use std::{fs, env};
use crate::inf;
use toml::{Value, toml};
use crate::{
    err_rec,
    stat_add_pkg,
    stat_create_database,
    stat_get_value,
    stat_rem_pkg,
    inssort,
    stat_dump_dat,
    uninstall,
};

pub fn rebuild(noconfirm: bool) {
    let homepath = env::var("HOME").unwrap();
    let file = format!("{}/.config/ame/pkgs.toml", env::var("HOME").unwrap());
    let mut database = String::new();
    database = fs::read_to_string(&file).expect("Can't Open Database");
    inf("installing crystal config".to_string());

    let db_parsed = database.parse::<toml::Value>().expect("Invalid Database");
    let mut pkgs = Vec::new();
    for entry in db_parsed.as_table() {
        for (key, value) in &*entry {
            let mut tempvec = Vec::new();
            println!("{}", key);
            println!("{}", format!("{}",value).replace("update = ", ""));
            tempvec.push(key.to_string());
            tempvec.push(format!("{}",value).replace("update = ", ""));
            pkgs.push(tempvec);
        }
    }
    let mut pkgs_to_add: Vec<Vec<String>> = Vec::new();
    let mut pkgs_to_install: Vec<String> = Vec::new();
    for i in pkgs {
        if !stat_get_value(&i[0], "name") {
            let mut tempvec = Vec::new();
            tempvec.push(i[0].to_string());
            tempvec.push(i[1].to_string());
            pkgs_to_add.push(tempvec);
            pkgs_to_install.push(i[0].to_string());
        }
    }
    let mut config_no_change = 0;
    if pkgs_to_install.len() > 0 {
        inf(format!("Installing {}", pkgs_to_install.join(", ")));
        inssort(noconfirm, false, pkgs_to_install);
        for i in pkgs_to_add {
            stat_add_pkg(&i[1], &i[0]);
        }
        config_no_change += 1;
    }
    let dat_pkgs = stat_dump_dat();

    let mut pkgs = Vec::new();
    for entry in db_parsed.as_table() {
        for (key, value) in &*entry {
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
    if pkgs_to_remove.len() > 0 {
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