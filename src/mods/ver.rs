use crate::inf;
use ansi_term::Colour;

pub fn ver() {
    println!("");
    inf(format!("ame - v2.4.0"));
    println!("");
    inf(format!("Contributors:"));
    println!("- axtlos <axtlos@salyut.one>");
    println!("- jnats <jnats@salyut.one>");
    println!("- jasio <jasiobene@icloud.com>");
    println!("- generic <mdc028@bucknell.edu>");
    println!("");
    inf(format!(
        "This software is licensed under the BSD 3-Clause license."
    ));
    inf(format!("All source code is available at:"));
    println!("");
    println!(
        "{}",
        Colour::Purple
            .bold()
            .paint("https://github.com/crystal-linux/ame")
    );
    println!("");
}
