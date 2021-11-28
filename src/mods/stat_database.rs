use crate::{err_unrec, inf};
use std::{fs, env};

pub fn stat_create_database() {
    let homepath = env::var("HOME").unwrap();
    let file = format!("{}/.local/share/ame/aur_pkgs.db", env::var("HOME").unwrap());
    if !std::path::Path::new(&format!("{}/.local/share/ame/", env::var("HOME").unwrap())).is_dir() {
        let _cdar = fs::create_dir_all(format!("/{}/.local/ame/",homepath));
            match _cdar {
                Ok(_) => {
                    inf(format!("Created path for database (previously missing)"))
                }
                Err(_) => {
                    err_unrec(format!("Couldn't create path for database (~/.local/rhare/ame)"))
                }
            }
    }
    let connection = sqlite::open(file).unwrap();
    connection.execute(
        "
        CREATE TABLE static_pkgs (name TEXT, pin INTEGER);
        ",
    )
    .unwrap();
} 

pub fn stat_get_value(pkg: &str, sear_value: &str) -> bool {
    let file = format!("{}/.local/share/ame/aur_pkgs.db", env::var("HOME").unwrap());
    let connection = sqlite::open(file).unwrap();
    let mut return_val = false;
    //println!("{}", pkg);
    match sear_value {
        "name" => {
            let result = connection.iterate(format!("SELECT name FROM static_pkgs WHERE name = \"{}\";",&pkg), |pairs| { 
                for &(column, value) in pairs.iter() {
                    return_val = true;
                }
                return_val
            }
            );
            match result {
                Ok(_) => {},
                Err(_) => err_unrec("Couldn't get value from database".to_string()),
            }
            if return_val == true {
                return true;
            } else {
                return false;
            }
        },
        "update" => {
            let result = connection.iterate(format!("SELECT pin FROM static_pkgs WHERE name = \"{}\";",&pkg), |pairs| { 
                for &(column, value) in pairs.iter() {
                    return_val = true;
                }
                return_val
            }
            );
            match result {
                Ok(_) => {},
                Err(_) => err_unrec("Couldn't get value from database".to_string()),
            }
            if return_val == true {
                return true;
            } else {
                return false;
            }
        },
        _ => {
            return_val = false
        }
    }
    return return_val;
}

pub fn stat_rem_pkg(static_pkgs: &Vec<String>) {
    let file = format!("{}/.local/share/ame/aur_pkgs.db", env::var("HOME").unwrap());
    let connection = sqlite::open(file).unwrap();

    for i in static_pkgs {
        let result = connection.execute(
            format!("
            DELETE FROM static_pkgs WHERE name = \"{}\";
            ", i),
        );
        match result{
            Ok(_) => {
                inf(format!("Removed {} from database", i))
            }
            Err(_) => {
                err_unrec(format!("Couldn't remove {} from database", i))
            }
        }
    }
}

pub fn stat_add_pkg(update: &str, pkg: &str) {
    let file = format!("{}/.local/share/ame/aur_pkgs.db", env::var("HOME").unwrap());
    let connection = sqlite::open(file).unwrap();
    let pin = if update == "true" { 1 } else { 0 };
    let result = connection.execute(
        format!("
        INSERT INTO static_pkgs (name, pin) VALUES (\"{}\", {});
        ", pkg, pin),
    );
    match result{
        Ok(_) => {
            inf(format!("Added {} to database", pkg))
        }
        Err(_) => {
            err_unrec(format!("Couldn't add {} to database", pkg))
        }
    }
}