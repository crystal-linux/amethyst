use std::ops::Deref;

pub fn search(pkg: &str) {
    let results = raur::search(&pkg);
    for res in &results {
        println!("{} {}\n   {}", res[0].name, res[0].version, res[0].description.as_ref().map_or("n/a", String::deref));
    }
}