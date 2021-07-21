use std::process::Command;
use json;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use curl::easy::Easy;
use std::io::{stdout, Write};

pub fn search(pkg: &str) {
    let url = format!("https://aur.archlinux.org/rpc/?v=5&type=search&arg={}",&pkg);
    
    let mut easy = Easy::new();
    easy.url(&url).unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();

    let output = easy.response_code().unwrap();
    println!("{}", output);

    Command::new("pacman").arg("-Ss").arg(&pkg).spawn().expect("Failed to run pacman");
}  