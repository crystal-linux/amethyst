use toml_edit::{Document, value};
use std::io::{Read, Write};


pub fn addPkg(pkgs: Vec<String>) {
    let file =  format!("{}/.local/ame/aurPkgs.db", std::env::var("HOME").unwrap());
    let database = std::fs::read_to_string(&file).expect("cant open new configuration");

    let mut dbParsed = database.parse::<Document>().expect("invalid Database");
    for i in pkgs {
        let results = raur::search(&i);
        for res in &results { 
            for r in res {
                dbParsed[&i]["name"] = value(&r.name);
                dbParsed[&i]["version"] = value(&r.version);
            }
        }
    }
    print!("{}",dbParsed);
}