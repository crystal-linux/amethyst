use std::env;
use std::path::Path;

use rusqlite::Connection;

use crate::{log, Options};

pub fn remove(pkg: &str, options: Options) {
    let conn = Connection::open(Path::new(&format!(
        "{}/.local/share/ame/db.sqlite",
        env::var("HOME").unwrap()
    )))
    .expect("Couldn't connect to database");

    let verbosity = options.verbosity;

    if verbosity >= 1 {
        log(format!("Removing package {} from database", pkg));
    }

    conn.execute(
        "DELETE FROM packages 
             WHERE EXISTS
                (SELECT *
                 FROM packages
                 WHERE name = ?);",
        [pkg],
    )
    .expect("Couldn't delete package from database");
}
