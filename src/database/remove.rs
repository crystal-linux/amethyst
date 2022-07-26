use crate::{log, Options};

use super::get_database_connection;

pub fn remove(pkg: &str, options: Options) {
    // Initialise database connection
    let conn = get_database_connection();

    // Initialise variables
    let verbosity = options.verbosity;

    if verbosity >= 1 {
        log!("Removing package {} from database", pkg);
    }

    // Remove the package from the database
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
