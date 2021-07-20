use std::process::Command;
use json;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

pub fn search(pkg: &str) {
    let homedir = env::home_dir();
    match homedir {
        Some(homedirr) => println!("{}",homedirr.display()),
        None => panic!("could not read home dir")
    }
    Command::new("pacman").arg("-Ss").arg(&pkg).spawn();
    let search_path = Path::new("/Users/ali/search.json"); //TODO: make it use a relative path to the search
    let display = search_path.display();

    let mut searchfile = match File::open(&search_path) {
        Err(why) => panic!("couldnt read {}: {}", display, why),
        Ok(searchfile) => searchfile,
    };

    let mut content = String::new();
    match searchfile.read_to_string(&mut content) {
        Err(why) => panic!("Couldnt read {}: {}", display, why),
        Ok(_) => print!("{}", content),
    };
}  