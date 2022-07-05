use std::{env, path::PathBuf};

pub mod add;
pub mod initialise;
pub mod query;
pub mod remove;

pub use add::*;
pub use initialise::*;
pub use query::*;
pub use remove::*;
use rusqlite::Connection;

fn get_database_connection() -> Connection {
    let db_path = format!("{}/.local/share/ame/db.sqlite", env::var("HOME").unwrap());
    let conn = Connection::open(PathBuf::from(db_path)).expect("Couldn't connect to database");
    conn
}
