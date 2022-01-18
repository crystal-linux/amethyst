use crate::Options;
use rusqlite::Connection;
use std::env;
use std::path::Path;

pub fn remove(pkg: &str, options: Options) {
    let conn = Connection::open(Path::new(&format!(
        "{}/.local/share/ame/db.sqlite",
        env::var("HOME").unwrap()
    )))
    .expect("Couldn't connect to database");

    let verbosity = options.verbosity;

    if verbosity >= 1 {
        eprintln!("Removing package {} from database", pkg);
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
