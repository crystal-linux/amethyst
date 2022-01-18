use crate::Options;

mod add;

pub fn add(a: String, options: Options) {
    add::add(a, options);
}
