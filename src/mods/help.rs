use crate::mods::strs::{inf, err_rec};

pub fn help() {
    println!("");
    inf(format!("Usage:"));
    println!("
ame -S(n)   / ins         <pkg> - install a package
ame -R(n)   / -Rs(n) / rm <pkg> - remove  a package
ame -Syu(n) / upg               - upgrade all packages to latest version
ame -Ss     / sea         <pkg> - search for a package
ame -Sa     / aursea      <pkg> - search for a package in the aur
ame -Sr     / repsea      <pkg> - search for a package in the repos
ame -v      / ver               - contributors and version info");
    println!("");
    err_rec(format!("Appending 'n' where (n) is present passes '--noconfirm' to pacman. Use at your own risk."));
    println!("");

}
