use std::fmt::Display;

pub trait Printable: Display {
    fn to_print_string(&self) -> String {
        self.to_string()
    }
}
