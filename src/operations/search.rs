use crate::error::SilentUnwrap;
use crate::internal::rpc::rpcsearch;
use crate::{log, pacman, Options};

pub fn aur_search(a: &str, options: Options) {
    let verbosity = options.verbosity;
    let res = rpcsearch(a.to_string());

    for r in &res.results {
        println!(
            "aur/{} {}\n    {}",
            r.name,
            r.version,
            r.description
                .as_ref()
                .unwrap_or(&"No description".to_string())
        )
    }

    if verbosity >= 1 {
        log(format!(
            "Found {} resuls for \"{}\" in AUR",
            res.resultcount, a
        ));
    }
}

pub fn repo_search(a: &str, options: Options) {
    let verbosity = options.verbosity;
    let output = pacman(&["-Ss", a]).silent_unwrap();

    if verbosity >= 1 {
        log(format!(
            "Found {} results for \"{}\" in repos",
            &output.split('\n').count() / 2,
            &a
        ));
    }
}
