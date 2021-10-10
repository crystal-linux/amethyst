use regex::bytes;
use toml_edit::{Document, value};
use std::io::{Read, Write, Error};
use std::fs::File;

pub fn remPkg(pkgs: &Vec<String>) {
    let file = format!("{}/.local/ame/aurPkgs.db", std::env::var("HOME").unwrap());
    let mut database = std::fs::read_to_string(&file).expect("cant open database");

    let mut updateDatabase = database;
    for i in pkgs { 
        if updateDatabase.contains(i) {
            let results = raur::search(&i);
            for res in &results { 
                let databaseEntry = format!("{} = {{ name = \"{}\", version = \"{}\"}}\n",&res[0].name, &res[0].name, &res[0].version);
                updateDatabase = format!("{}",updateDatabase.replace(&databaseEntry, ""));
            }
        }
    }
    let fileAsPath = File::create(std::path::Path::new(&file)).unwrap();
    write!(&fileAsPath, "{}", updateDatabase);

}

pub fn addPkg(fromRepo: bool, pkg: &str) -> Result<(), Error> {
    let file =  format!("{}/.local/ame/aurPkgs.db", std::env::var("HOME").unwrap());
    let database = std::fs::read_to_string(&file).expect("cant open database");
    let mut fileAsPath = File::create(std::path::Path::new(&file))?;

    let mut dbParsed = database.parse::<Document>().expect("invalid Database");
    if fromRepo == false {
        let results = raur::search(&pkg);
        for res in &results { 
            for r in res {
                dbParsed[&r.name]["name"] = value(&r.name);
                dbParsed[&r.name]["version"] = value(&r.version);
            }
        }
    } else {
        dbParsed[&pkg]["name"] = value(pkg);
        dbParsed[&pkg]["version"] = value(pkg);
    }
    print!("{}",dbParsed);
    fileAsPath.write_all(format!("{}",dbParsed).as_bytes()).unwrap();
    Ok(())
}