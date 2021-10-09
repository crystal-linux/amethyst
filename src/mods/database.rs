use regex::bytes;
use toml_edit::{Document, value};
use std::io::{Read, Write, Error};
use std::fs::File;

pub fn remPkg(pkgs: Vec<String>) {
    let file = format!("{}/.local/ame/aurPkgs.db", std::env::var("HOME").unwrap());
    let mut database = std::fs::read_to_string(&file).expect("cant open database");

    let mut updateDatabase = database;
    for i in pkgs { 
        let results = raur::search(&i);
        for res in &results { 
            let databaseEntry = format!("{} = {{ name = \"{}\", version = \"{}\"}}\n",&i, &res[0].name, &res[0].version);
            updateDatabase = format!("{}",updateDatabase.replace(&databaseEntry, ""));
        }
    }
    let fileAsPath = File::create(std::path::Path::new(&file)).unwrap();
    write!(&fileAsPath, "{}", updateDatabase);

}

pub fn addPkg(pkgs: Vec<String>) -> Result<(), Error> {
    let file =  format!("{}/.local/ame/aurPkgs.db", std::env::var("HOME").unwrap());
    let database = std::fs::read_to_string(&file).expect("cant open database");
    let mut fileAsPath = File::create(std::path::Path::new(&file))?;

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
    fileAsPath.write_all(format!("{}",dbParsed).as_bytes()).unwrap();
    Ok(())
}