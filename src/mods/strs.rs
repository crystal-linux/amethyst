use ansi_term::Colour;
use std::{process, env};
use uwuizer::*;

pub fn inf(a: std::string::String){
    if env::var("AME_UWU").unwrap_or("n/a".to_string()) == "YES" {
        println!("{} {}",
                 Colour::Purple.paint("❖"),
                 Colour::White.paint(uwuize!(&a)));
    } else {
        println!("{} {}",
                 Colour::Purple.paint("❖"),
                 Colour::White.paint(a));
    }
}

pub fn sec(a: std::string::String){
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

pub fn succ(a: std::string::String) {
    if env::var("AME_UWU").unwrap_or("n/a".to_string()) == "YES" {
        println!("{} {}",
                 Colour::Green.bold().paint("✓"),
                 Colour::Green.paint(uwuize!(&a)));
    } else {
        println!("{} {}",
                 Colour::Green.bold().paint("✓"),
                 Colour::Green.paint(&a));
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
