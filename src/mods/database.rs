use crate::{err_unrec, inf};
use std::{env, fs};

pub fn create_database() {
    let homepath = env::var("HOME").unwrap();
    let file = format!("{}/.local/share/ame/aur_pkgs.db", env::var("HOME").unwrap());
    if !std::path::Path::new(&format!("{}/.local/share/ame/", env::var("HOME").unwrap())).is_dir() {
        let _cdar = fs::create_dir_all(format!("/{}/.local/ame/", homepath));
        match _cdar {
            Ok(_) => {
                inf("Created path for database (previously missing)".to_string());
            }
            Err(_) => {
                err_unrec("Couldn't create path for database (~/.local/rhare/ame)".to_string())
            }
        }
    }
    let connection = sqlite::open(file).unwrap();
    connection
        .execute(
            "
        CREATE TABLE pkgs (name TEXT, version TEXT);
        ",
        )
        .unwrap();
}

pub fn get_value(pkg: &str, sear_value: &str) -> String {
    let file = format!("{}/.local/share/ame/aur_pkgs.db", env::var("HOME").unwrap());
    let connection = sqlite::open(file).unwrap();
    let mut return_val = String::new();
    match sear_value {
        "name" => {
            let result = connection.iterate(
                format!("SELECT name FROM pkgs WHERE name = {};", &pkg),
                |pairs| {
                    for &(_column, value) in pairs.iter() {
                        return_val = value.unwrap().to_string();
                    }
                    true
                },
            );
            match result {
                Ok(_) => {}
                Err(_) => err_unrec("Couldn't get value from database".to_string()),
            }
        }
        "version" => {
            let result = connection.iterate(
                format!("SELECT version FROM pkgs WHERE name = {};", &pkg),
                |pairs| {
                    for &(_column, value) in pairs.iter() {
                        return_val = value.unwrap().to_string();
                    }
                    true
                },
            );
            match result {
                Ok(_) => {}
                Err(_) => err_unrec("Couldn't get value from database".to_string()),
            }
        }
        _ => {
            return_val = "error".to_string();
        }
    }
    return_val
}

pub fn rem_pkg(pkgs: &[String]) {
    let file = format!("{}/.local/share/ame/aur_pkgs.db", env::var("HOME").unwrap());
    let connection = sqlite::open(file).unwrap();

    for i in pkgs {
        let result = connection.execute(format!(
            "
            DELETE FROM pkgs WHERE name = \"{}\";
            ",
            i
        ));
        match result {
            Ok(_) => inf(format!("Removed {} from database", i)),
            Err(_) => err_unrec(format!("Couldn't remove {} from database", i)),
        }
    }
}

pub fn add_pkg(from_repo: bool, pkgs: &Vec<&str>) {
    for pkg in pkgs {
        let file = format!("{}/.local/share/ame/aur_pkgs.db", env::var("HOME").unwrap());
        let connection = sqlite::open(file).unwrap();
        let results = raur::search(&pkg);
        let mut package_name = String::new();
        let mut package_version = String::new();
        for res in &results {
            package_name = res[0].name.to_string();
            package_version = res[0].version.to_string();
        }
        if from_repo == false {
            let result = connection.execute(format!(
                "
                INSERT INTO pkgs (name, version) VALUES (\"{}\", \"{}\");
                ",
                package_name, package_version
            ));
            match result {
                Ok(_) => inf(format!("Added {} to database", package_name)),
                Err(_) => err_unrec(format!("Couldn't add {} to database", package_name)),
            }
        } else {
            let result = connection.execute(format!(
                "
                INSERT INTO pkgs (name, version) VALUES (\"{}\", \"{}\");
                ",
                pkg, "from_repo".to_string()
            ));
            match result {
                Ok(_) => inf(format!("Added {} to database", package_name)),
                Err(_) => err_unrec(format!("Couldn't add {} to database", package_name)),
            }
        }
    }
}
