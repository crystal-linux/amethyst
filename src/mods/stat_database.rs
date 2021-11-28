use crate::{err_unrec, inf};
use std::{fs, env};

pub fn stat_create_database() {
    let homepath = env::var("HOME").unwrap();
    let file = format!("{}/.local/share/ame/aur_pkgs.db", env::var("HOME").unwrap());
    if !std::path::Path::new(&format!("{}/.local/share/ame/", env::var("HOME").unwrap())).is_dir() {
        let _cdar = fs::create_dir_all(format!("/{}/.local/ame/",homepath));
            match _cdar {
                Ok(_) => {
                    inf("Created path for database (previously missing)".to_string());
                }
                Err(_) => {
                    err_unrec("Couldn't create path for database (~/.local/share/ame)".to_string())
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

pub fn stat_dump_dat() -> Vec<String> {
    let homepath = env::var("HOME").unwrap();
    let file = format!("{}/.local/share/ame/aur_pkgs.db", env::var("HOME").unwrap());
    let connection = sqlite::open(file).unwrap();
    let mut dat_pkgs = Vec::new();
    let result = connection
        .iterate("SELECT name FROM static_pkgs", |pairs| {
            for &(column, value) in pairs.iter() {
                dat_pkgs.push(value.unwrap().to_string());
            }
            true
        });
    match result {
        Ok(_) => {
            //nf("Dumped static packages".to_string());
        }
        Err(_) => {
            err_unrec("Couldn't dump packages from database".to_string())
        }
    }
    return dat_pkgs;
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
            return return_val == true;
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
            return return_val == true
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
    print!("{:?}",static_pkgs);
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
                err_unrec(format!("Couldn't remove {} from database (static packages table)", i))
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