use crate::{err_unrec, inf};
use std::env;

pub fn stat_dump_dat() -> Vec<String> {
    let file = format!("{}/.local/share/ame/aur_pkgs.db", env::var("HOME").unwrap());
    let connection = sqlite::open(file).unwrap();
    let mut dat_pkgs = Vec::new();
    let result = connection.iterate("SELECT name FROM static_pkgs", |pairs| {
        for &(_column, value) in pairs.iter() {
            dat_pkgs.push(value.unwrap().to_string());
        }
        true
    });
    match result {
        Ok(_) => {
            //inf("Dumped static packages".to_string());
        }
        Err(_) => err_unrec("Couldn't dump packages from database".to_string()),
    }
    dat_pkgs
}

pub fn stat_get_value(pkg: &str, sear_value: &str) -> bool {
    let file = format!("{}/.local/share/ame/aur_pkgs.db", env::var("HOME").unwrap());
    let connection = sqlite::open(file).unwrap();
    let mut return_val = false;
    match sear_value {
        "name" => {
            let result = connection.iterate(
                format!("SELECT name FROM static_pkgs WHERE name = \"{}\";", &pkg),
                |pairs| {
                    for &(_column, _value) in pairs.iter() {
                        return_val = true;
                    }
                    return_val
                },
            );
            match result {
                Ok(_) => {}
                Err(_) => err_unrec("Couldn't get value from database".to_string()),
            }
            return return_val;
        }
        "update" => {
            let result = connection.iterate(
                format!("SELECT pin FROM static_pkgs WHERE name = \"{}\";", &pkg),
                |pairs| {
                    for &(_column, _value) in pairs.iter() {
                        return_val = true;
                    }
                    return_val
                },
            );
            match result {
                Ok(_) => {}
                Err(_) => err_unrec("Couldn't get value from database".to_string()),
            }
            return return_val;
        }
        _ => return_val = false,
    }
    return_val
}

pub fn stat_rem_pkg(static_pkgs: &[String]) {
    let file = format!("{}/.local/share/ame/aur_pkgs.db", env::var("HOME").unwrap());
    let connection = sqlite::open(file).unwrap();
    print!("{:?}", static_pkgs);
    for i in static_pkgs {
        let result = connection.execute(format!(
            "
            DELETE FROM static_pkgs WHERE name = \"{}\";
            ",
            i
        ));
        match result {
            Ok(_) => inf(format!("Removed {} from database", i)),
            Err(_) => err_unrec(format!(
                "Couldn't remove {} from database (static packages table)",
                i
            )),
        }
    }
}

pub fn stat_add_pkg(update: &str, pkg: &str) {
    let file = format!("{}/.local/share/ame/aur_pkgs.db", env::var("HOME").unwrap());
    let connection = sqlite::open(file).unwrap();
    let pin = if update == "true" { 1 } else { 0 };
    let result = connection.execute(format!(
        "
        INSERT INTO static_pkgs (name, pin) VALUES (\"{}\", {});
        ",
        pkg, pin
    ));
    match result {
        Ok(_) => inf(format!("Added {} to database", pkg)),
        Err(_) => err_unrec(format!("Couldn't add {} to database", pkg)),
    }
}
