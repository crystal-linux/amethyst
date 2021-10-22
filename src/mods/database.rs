use crate::{err_unrec, inf};
use std::{fs, io::{Error, Write}, env, path};
use toml_edit::{value, Document};
use crate::mods::strs::{err_rec};

pub fn get_value(pkg: &str, sear_value: &str) -> String {
    let homepath = env::var("HOME").unwrap();
    let file = format!("{}/.local/ame/aurPkgs.db", env::var("HOME").unwrap());
    let mut database = String::new();
    match path::Path::new(&file).exists() {
        true => {
            database = fs::read_to_string(&file).expect("Can't Open Database");
        }
        false => {
            let _cdar = fs::create_dir_all(format!("/{}/.local/ame/",homepath));
            match _cdar {
                Ok(_) => {
                    inf(format!("Created path for database (previously missing)"))
                }
                Err(_) => {
                    err_unrec(format!("Couldn't create path for database (~/.local/ame)"))
                }
            }
            err_rec(String::from("Datbase wasn't found, creating new one"));
            let _dbfile = fs::File::create(&file);
            match _dbfile {
                Ok(_) => {
                    inf(format!("Created empty database (previously missing)"))
                }
                Err(_) => {
                    err_unrec(format!("Couldn't create database"))
                }
            }
        }
    }
    let db_parsed = database.parse::<toml::Value>().expect("Invalid Database");

    let mut return_val = String::new();
    for entry in db_parsed.as_table() {
        for (key, value) in &*entry {
            if key.contains(pkg) {
                let results = raur::search(format!("{}",key));
                for res in results {
                    match sear_value {
                        "name" => {
                            return_val = value
                                .to_string()
                                .replace("name", "")
                                .replace("version", "")
                                .replace(" = ", "")
                                .replace("\"", "")
                                .replace(format!("{}", &res[0].version.to_string()).as_str(), "");
                        }
                        "version" => {
                            return_val = value
                                .to_string()
                                .replace("name", "")
                                .replace("version", "")
                                .replace(" = ", "")
                                .replace("\"", "")
                                .replace(format!("{}", &res[0].name.to_string()).as_str(), "");
                        }
                        _ => {
                            err_unrec(format!(""));
                        }
                    }
                }
            }
        }
    }
    return return_val;
}

pub fn rem_pkg(pkgs: &Vec<String>) {
    let homepath = env::var("HOME").unwrap();
    let file = format!("{}/.local/ame/aurPkgs.db", env::var("HOME").unwrap());
    let mut database = String::new();
    match path::Path::new(&file).exists() {
        true => {
            database = fs::read_to_string(&file).expect("Can't Open Database");
        }
        false => {
            let _cdar = fs::create_dir_all(format!("/{}/.local/ame/",homepath));
            match _cdar {
                Ok(_) => {
                    inf(format!("Created path for database (previously missing)"))
                }
                Err(_) => {
                    err_unrec(format!("Couldn't create path for database (~/.local/ame)"))
                }
            }
            err_rec(String::from("Datbase wasn't found, creating new one"));
            let _dbfile = fs::File::create(&file);
            match _dbfile {
                Ok(_) => {
                    inf(format!("Created empty database (previously missing)"))
                }
                Err(_) => {
                    err_unrec(format!("Couldn't create database"))
                }
            }
        }
    }

    let mut update_database = database;
    for i in pkgs {
        if update_database.contains(i) {
            let results = raur::search(&i);
            for res in &results {
                let database_entry = format!(
                    "{} = {{ name = \"{}\", version = \"{}\"}}\n",
                    &res[0].name, &res[0].name, &res[0].version
                );
                update_database = format!("{}", update_database.replace(&database_entry, ""));
            }
        }
    }
    let file_as_path = fs::File::create(path::Path::new(&file)).unwrap();
    let db_update_res = write!(&file_as_path, "{}", update_database);
    match db_update_res {
        Ok(_) => inf(format!("Database update successful")),
        Err(_) => err_unrec(format!("Couldn't update database")),
    }
}

pub fn add_pkg(from_repo: bool, pkg: &str) -> Result<(), Error> {
    let homepath = env::var("HOME").unwrap();
    let file = format!("{}/.local/ame/aurPkgs.db", env::var("HOME").unwrap());
    let mut database = String::new();
    match path::Path::new(&file).exists() {
        true => {
            database = fs::read_to_string(&file).expect("Can't Open Database");
        }
        false => {
            let _cdar = fs::create_dir_all(format!("/{}/.local/ame/",homepath));
            match _cdar {
                Ok(_) => {
                    inf(format!("Created path for database (previously missing)"))
                }
                Err(_) => {
                    err_unrec(format!("Couldn't create path for database (~/.local/ame)"))
                }
            }
            err_rec(String::from("Datbase wasn't found, creating new one"));
            let _dbfile = fs::File::create(&file);
            match _dbfile {
                Ok(_) => {
                    inf(format!("Created empty database (previously missing)"))
                }
                Err(_) => {
                    err_unrec(format!("Couldn't create database"))
                }
            }
        }
    }
    let mut db_parsed = database.parse::<Document>().expect("Invalid Database");
    let mut file_as_path = fs::File::create(path::Path::new(&file))?;
    if from_repo == false {
        let results = raur::search(&pkg);
        for res in &results {
            db_parsed[&res[0].name]["name"] = value(&res[0].name);
            db_parsed[&res[0].name]["version"] = value(&res[0].version);
        }
    } else {
        db_parsed[&pkg]["name"] = value(pkg);
        db_parsed[&pkg]["version"] = value(pkg);
    }
    file_as_path
        .write_all(format!("{}", db_parsed).as_bytes())
        .unwrap();
    Ok(())
}
