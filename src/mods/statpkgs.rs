use std::{fs, env};
use crate::inf;
use toml::{Value, toml};
use crate::{
    stat_add_pkg,
    stat_create_database,
    stat_get_value,
    stat_rem_pkg,
};

pub fn rebuild(noconfirm: bool) {
    let homepath = env::var("HOME").unwrap();
    let file = format!("{}/.config/ame/pkgs.toml", env::var("HOME").unwrap());
    let mut database = String::new();
    database = fs::read_to_string(&file).expect("Can't Open Database");
    inf(format!("installing crystal config"));

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
    let mut pkgs_to_add: Vec<String> = Vec::new();
    for i in pkgs {
        if !stat_get_value(&i[0], "name") {
            pkgs_to_add.push(i[0].to_string());
        }
    }
    inf(format!("Installing {}", pkgs_to_add.join(", ")));
}