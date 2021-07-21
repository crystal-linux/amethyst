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
    easy.url("https://www.rust-lang.org/").unwrap();

    let mut data = Vec::new();
let mut handle = Easy::new();
handle.url(&url).unwrap();
{
    let mut transfer = handle.transfer();
    transfer.write_function(|new_data| {
        data.extend_from_slice(new_data);
        Ok(new_data.len())
    }).unwrap();
    transfer.perform().unwrap();
}
//println!("{:?}", data);

    let s = String::from_utf8_lossy(&data);
    println!("result: {}", s);

    Command::new("pacman").arg("-Ss").arg(&pkg).spawn().expect("Failed to run pacman");

    //let parsed = json::parse(&output)
}  