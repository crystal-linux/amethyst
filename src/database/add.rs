use crate::internal::exit_code::AppExitCode;
use crate::internal::rpc::Package;
use crate::{crash, log, Options};

use super::get_database_connection;

pub fn add(pkg: Package, options: Options) {
    // Initialise database connection
    let conn = get_database_connection();

    // Log the package name
    if options.verbosity >= 1 {
        log!("Adding package {} to database", pkg.name);
    }

    // Push the package to the database
    let pkg_description = pkg
        .description
        .unwrap_or_else(|| "No description found.".parse().unwrap());
    conn.execute("INSERT OR REPLACE INTO packages (name, version, description, depends, make_depends) VALUES (?1, ?2, ?3, ?4, ?5)",
                 [&pkg.name, &pkg.version, &pkg_description, &pkg.depends.join(" "), &pkg.make_depends.join(" ")],
    ).unwrap_or_else(|e|
        crash!(AppExitCode::FailedAddingPkg, "Failed adding package {} to the database: {}", pkg.name, e) 
    );
}
