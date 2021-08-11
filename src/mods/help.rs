pub fn help() {
    println!("\
Usage:\n
ame -S <pkg>  - install a package
ame -f <pkg>  - install a package via flatpak
ame -R <pkg>  - remove  a package
ame -Syu      - system upgrade
ame -Ss <pkg> - search for a package
ame -Cc       - clear package cache")
}