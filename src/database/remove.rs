use crate::{log, Options};

use super::get_database_connection;

pub fn remove(pkg: &str, options: Options) {
    let conn = get_database_connection();

    let verbosity = options.verbosity;

    if verbosity >= 1 {
        log!("Removing package {} from database", pkg);
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
