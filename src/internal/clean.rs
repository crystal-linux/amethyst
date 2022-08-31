/// Strips packages from versioning and other extraneous information.
pub fn clean(a: &[String]) -> Vec<String> {
    // Strip versioning from package names
    let cleaned = a
        .iter()
        .map(|name| {
            name.split_once("=")
                .map(|n| n.0.to_string())
                .unwrap_or(name.to_string())
        })
        .collect();

    tracing::debug!("Cleaned: {:?}\nInto: {:?}", a, cleaned);

    cleaned
}
