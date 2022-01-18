use crate::Options;
use regex::Regex;

pub fn clean(a: &[String], options: Options) -> Vec<String> {
    let r = Regex::new(r"(\S+)((?:>=|<=|>|<)\S+$)").unwrap();
    let mut cleaned: Vec<String> = vec![];
    let verbosity = options.verbosity;

    for b in a {
        if r.captures_iter(b).count() > 0 {
            let c = r.captures(b).unwrap().get(1).map_or("", |m| m.as_str());
            cleaned.push(c.to_string());
        } else {
            cleaned.push(b.to_string());
        }
    }

    match verbosity {
        0 => {}
        1 => {
            eprintln!("Cleaned: {:?}\nInto: {:?}", a, cleaned);
        }
        _ => {
            eprintln!("Cleaned:");
            for b in a {
                eprintln!("{}", b);
            }
            eprintln!("Into:");
            for c in &cleaned {
                eprintln!("{}", c);
            }
        }
    }

    cleaned
}
