use ansi_term::Colour;
use std::{process, env};
use uwuizer::*;

pub fn inf(a: std::string::String){
    if env::var("AME_UWU").unwrap_or("n/a".to_string()) == "YES" {
        println!("{} {}",
                 Colour::Purple.bold().paint("❖"),
                 Colour::White.bold().paint(uwuize!(&a)));
    } else {
        println!("{} {}",
                 Colour::Purple.bold().paint("❖"),
                 Colour::White.bold().paint(a));
    }
}

pub fn err_unrec(a: std::string::String) {
    if env::var("AME_UWU").unwrap_or("n/a".to_string()) == "YES" {
        println!("{} {} {}",
                 Colour::Red.bold().paint(uwuize!("✖ Unrecoverable error:")),
                 Colour::Red.paint(uwuize!(&a)),
                 Colour::Red.bold().paint(uwuize!("Terminating.")));
        process::exit(1);
    } else {
        println!("{} {} {}",
                 Colour::Red.bold().paint("✖ Unrecoverable error:"),
                 Colour::Red.paint(a),
                 Colour::Red.bold().paint("Terminating."));
        process::exit(1);
    }
}

// we havent actually used this one yet

pub fn err_rec(a: std::string::String) {
    if env::var("AME_UWU").unwrap_or("n/a".to_string()) == "YES" {
        println!("{} {}",
                 Colour::Yellow.bold().paint(uwuize!("⚠ WARNING:")),
                 Colour::Yellow.paint(uwuize!(&a)));
    } else {
        println!("{} {}",
                 Colour::Yellow.bold().paint("⚠ WARNING:"),
                 Colour::Yellow.paint(a));
    }
}
