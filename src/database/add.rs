use std::env;
use std::path::Path;

use rusqlite::Connection;

use crate::{crash, log, Options};
use crate::internal::rpc::Package;

pub fn add(pkg: Package, options: Options) {
    let conn = Connection::open(Path::new(&format!(
        "{}/.local/share/ame/db.sqlite",
        env::var("HOME").unwrap()
    )))
    .expect("Couldn't connect to database");

    if options.verbosity >= 1 {
        log(format!("Adding package {} to database", pkg.name));
    }

    conn.execute("INSERT OR REPLACE INTO packages (name, version, description, depends, make_depends) VALUES (?1, ?2, ?3, ?4, ?5)",
                 [&pkg.name, &pkg.version, &pkg.description.unwrap_or_else(|| "No description found.".parse().unwrap()), &pkg.depends.join(" "), &pkg.make_depends.join(" ")],
    ).unwrap_or_else(|e| {
        crash(format!("Failed adding package {} to the database: {}", pkg.name, e), 2);
        1
    });
}