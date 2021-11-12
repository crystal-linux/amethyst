use ansi_term::Colour;
use std::{env, io, io::Write, process, string};
use uwuizer::*;

pub fn inf(a: string::String) { // info
    if env::var("AME_UWU").unwrap_or("n/a".to_string()) == "YES" {
        println!(
            "{} {}",
            Colour::Purple.paint("❖"),
            Colour::White.paint(uwuize!(&a))
        );
    } else {
        println!("{} {}", Colour::Purple.paint("❖"), Colour::White.paint(a));
    }
}

pub fn sec(a: string::String) { 
    if env::var("AME_UWU").unwrap_or("n/a".to_string()) == "YES" {
        println!(
            "{} {}",
            Colour::Purple.bold().paint("❖"),
            Colour::White.bold().paint(uwuize!(&a))
        );
    } else {
        println!(
            "{} {}",
            Colour::Purple.bold().paint("❖"),
            Colour::White.bold().paint(a)
        );
    }
}

pub fn succ(a: string::String) { // success
    if env::var("AME_UWU").unwrap_or("n/a".to_string()) == "YES" {
        println!(
            "{} {}",
            Colour::Green.bold().paint("✓"),
            Colour::Green.paint(uwuize!(&a))
        );
    } else {
        println!(
            "{} {}",
            Colour::Green.bold().paint("✓"),
            Colour::Green.paint(&a)
        );
    }
}

pub fn prompt(a: string::String) -> bool { // prompt
    if env::var("AME_UWU").unwrap_or("n/a".to_string()) == "YES" {
        print!(
            "{} {} {}",
            Colour::Purple.bold().paint("❖"),
            Colour::White.bold().paint(uwuize!(&a)),
            Colour::White.bold().paint("(Y/n): ")
        );
        io::stdout().flush().ok().expect("Couldn't flush stdout");
        let mut yn: String = String::new();
        let _ = std::io::stdin().read_line(&mut yn);
        if yn.trim() == "n" || yn.trim() == "N" || yn.trim() == "no" || yn.trim() == "No" {
            false
        } else {
            true
        }
    } else {
        print!(
            "{} {} {}",
            Colour::Purple.bold().paint("❖"),
            Colour::White.bold().paint(&a),
            Colour::White.bold().paint("(Y/n): ")
        );
        io::stdout().flush().ok().expect("Couldn't flush stdout");
        let mut yn: String = String::new();
        let _ = std::io::stdin().read_line(&mut yn);
        if yn.trim() == "n" || yn.trim() == "N" || yn.trim() == "no" || yn.trim() == "No" {
            false
        } else {
            true
        }
    }
}

pub fn err_unrec(a: string::String) { // unrecoverable error
    if env::var("AME_UWU").unwrap_or("n/a".to_string()) == "YES" {
        println!(
            "{} {} {}",
            Colour::Red.bold().paint(uwuize!("✖ Unrecoverable error:")),
            Colour::Red.paint(uwuize!(&a)),
            Colour::Red.bold().paint(uwuize!("Terminating."))
        );
        process::exit(1);
    } else {
        println!(
            "{} {} {}",
            Colour::Red.bold().paint("✖ Unrecoverable error:"),
            Colour::Red.paint(a),
            Colour::Red.bold().paint("Terminating.")
        );
        process::exit(1);
    }
}

pub fn err_rec(a: string::String) { // recoverable error
    if env::var("AME_UWU").unwrap_or("n/a".to_string()) == "YES" {
        println!(
            "{} {}",
            Colour::Yellow.bold().paint(uwuize!("⚠ WARNING:")),
            Colour::Yellow.paint(uwuize!(&a))
        );
    } else {
        println!(
            "{} {}",
            Colour::Yellow.bold().paint("⚠ WARNING:"),
            Colour::Yellow.paint(a)
        );
    }
}
