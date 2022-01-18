use crate::Options;

mod add;

#[allow(dead_code)]
pub fn add(a: String, options: Options) {
    add::add(a, options);
}
