pub fn help() {
    println!("\
Usage:\n
ame -S <pkg>  - install a package
ame -f <pkg>  - install a package via flatpak
ame -s <pkg>  - install a package via snap
ame -R <pkg>  - remove  a package
ame -Syu      - system upgrade
ame -Ss <pkg> - search for a package
ame -Sa <pkg> - search for a package over the aur
ame -Sr <pkg> - search for a package over the repos 
ame -Cc       - clear package cache")
}