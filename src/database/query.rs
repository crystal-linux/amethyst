use std::env;
use std::path::Path;

use rusqlite::Connection;

use crate::{log, Options};
use crate::internal::rpc::Package;

pub fn query(options: Options) -> Vec<Package> {
    let verbosity = options.verbosity;

    if verbosity >= 1 {
        log("Connecting to database".to_string());
    }

    let conn = Connection::open(Path::new(&format!(
        "{}/.local/share/ame/db.sqlite",
        env::var("HOME").unwrap()
    )))
        .expect("Couldn't connect to database");

    if verbosity >= 1 {
        log("Querying database for input".to_string());
    }

    let mut rs = conn.prepare("SELECT * FROM packages;").unwrap();
    let packages_iter = rs
        .query_map([], |row| {
            Ok(Package {
                name: row.get(0).unwrap(),
                version: row.get(1).unwrap(),
                description: row.get(2).unwrap(),
                depends: row
                    .get::<usize, String>(3)
                    .unwrap()
                    .split(' ')
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
                make_depends: row
                    .get::<usize, String>(4)
                    .unwrap()
                    .split(' ')
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            })
        })
        .expect("Couldn't query database for packages");

    if verbosity >= 1 {
        log("Retrieved results".to_string());
    }

    let mut results: Vec<Package> = vec![];

    for package in packages_iter {
        results.push(package.unwrap());
    }

    if verbosity >= 1 {
        log("Collected results".to_string());
    }

    results
}
