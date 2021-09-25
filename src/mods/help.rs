use crate::mods::strs::inf;

pub fn help() { // work on a proper error message, the python ame one isnt really better in my opinion
    inf(format!("Usage:"));
    println!("
ame -S   / -Sy / ins <pkg> - install a package
ame -R   / -Rs / rm  <pkg> - remove  a package
ame -Syu / upg             - upgrade all packages to latest version
ame -Ss  / sea       <pkg> - search for a package
ame -Sa  / aursea    <pkg> - search for a package in the aur
ame -Sr  / repsea    <pkg> - search for a package in the repos
ame -v   / -V  / ver       - contributors and version info
"
);
}
