use crate::internal::strings::info;

pub fn detect(a: String) {
    if a.contains(".pacnew") || a.contains(".new") {
        info("It appears that a program you have installed / upgraded has installed a .new/.pacnew config file. Please read over the pacman output and act on it accordingly".to_string());
    } else if a.contains(".old") {
        info("It appears that a program you have installed / upgraded has installed a .old config file. Please read over the pacman output and act on it accordingly".to_string());
    }
}
