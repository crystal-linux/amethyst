use std::env;
use std::path::Path;

use rusqlite::Connection;

use crate::internal::exit_code::AppExitCode;
use crate::{crash, log, Options};

pub fn init(options: Options) {
    let path = format!("{}/.local/share/ame/db.sqlite", env::var("HOME").unwrap());
    let dbpath = Path::new(&path);
    let verbosity = options.verbosity;

    if verbosity >= 1 {
        log!("Creating database at {}", &path);
    }

    let conn =
        Connection::open(dbpath).expect("Couldn't create database at ~/.local/share/ame/db.sqlite");

    if verbosity >= 1 {
        log!("Populating database with table");
    }

    conn.execute(
        "CREATE TABLE packages (
                name         TEXT PRIMARY KEY NOT NULL,
                version      TEXT NOT NULL,
                description  TEXT,
                depends      BLOB,
                make_depends BLOB
                )",
        [],
    )
    .unwrap_or_else(|e| {
        crash!(
            AppExitCode::FailedInitDb,
            "Couldn't initialise database: {}",
            e,
        )
    });
}
