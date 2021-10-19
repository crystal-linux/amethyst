use crate::mods::strs::{err_rec, inf};

pub fn help() {
    println!("");
    inf(format!("Usage:"));
    println!(
        "
ame -S(n)   / ins         <pkg> - install a package
ame -R(n)   / rm          <pkg> - remove a package
ame -Rs(n)  / purge       <pkg> - remove a package with it dependencies 
ame -Syu(n) / upg               - upgrade all packages to latest version
ame -Ss     / sea         <pkg> - search for a package
ame -Sa     / aursea      <pkg> - search for a package in the aur
ame -Sr     / repsea      <pkg> - search for a package in the repos
ame -v      / ver               - contributors and version info
ame -h      / help              - display this help message

ame <any valid pacman flags>    - passes said flags to be processed by pacman"
    );
    println!("");
    err_rec(format!("Appending 'n' where (n) is present passes '--noconfirm' to pacman. Use at your own risk. (alternatively, using '--noconfirm' as a flag works too.)"));
    println!("");
}
