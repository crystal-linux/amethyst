use crate::inf;
use ansi_term::Colour;
use clap::{self, crate_version};

pub fn ver() {
    // print version and contributors
    println!();
    inf(format!("ame - {}", crate_version!()));
    println!();
    inf("Contributors:".to_string());
    println!("- axtlos <axtlos@salyut.one>");
    println!("- jnats <jnats@salyut.one>");
    println!("- jasio <jasiobene@icloud.com>");
    println!("- generic <mdc028@bucknell.edu>");
    println!();
    inf("This software is licensed under the BSD 3-Clause license.".to_string());
    inf("All source code is available at:".to_string());
    println!();
    println!(
        "{}",
        Colour::Purple
            .bold()
            .paint("https://git.getcryst.al/crystal/ame")
    );
    println!();
}
