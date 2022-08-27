use regex::Regex;

use crate::{log, Options};

/// Strips packages from versioning and other extraneous information.
pub fn clean(a: &[String], options: Options) -> Vec<String> {
    // Strip versioning from package names
    let r = Regex::new(r"(\S+)((?:>=|<=|>|<|=\W)\S+$)").unwrap();
    let mut cleaned: Vec<String> = vec![];
    let verbosity = options.verbosity;

    // Push cleaned package names to vector
    for b in a {
        if r.captures_iter(b).count() > 0 {
            let c = r.captures(b).unwrap().get(1).map_or("", |m| m.as_str());
            cleaned.push(c.to_string());
        } else {
            cleaned.push(b.to_string());
        }
    }

    if verbosity >= 1 {
        log!("Cleaned: {:?}\nInto: {:?}", a, cleaned);
    }

    cleaned
}
