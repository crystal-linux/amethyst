use ansi_term::Colour;
use std::process;

pub fn inf(a: std::string::String){
    println!("{} {}",
             Colour::Purple.bold().paint("❖"),
             Colour::White.bold().paint(a));
}

pub fn err_unrec(a: std::string::String) {
    println!("{} {} {}",
             Colour::Red.bold().paint("✖ Unrecoverable error:"),
             Colour::Red.paint(a),
             Colour::Red.bold().paint("Terminating."));
    process::exit(1);
}

// we havent actually used this one yet

pub fn err_rec(a: std::string::String) {
    println!("{} {}",
             Colour::Yellow.bold().paint("⚠ WARNING:"),
             Colour::Yellow.paint(a));
}
