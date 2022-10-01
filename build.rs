use serde::Deserialize;
use std::path::PathBuf;

use cargo_toml::Manifest;

#[derive(Clone, Debug, Deserialize)]
struct Metadata {
    codename: String,
}

fn main() {
    let manifest = Manifest::<Metadata>::from_path_with_metadata(PathBuf::from("Cargo.toml"))
        .expect("Failed to read manifest (Cargo.toml)");

    if let Some(package) = manifest.package {
        if let Some(metadata) = package.metadata {
            println!("cargo:rustc-env=AMETHYST_CODENAME={}", metadata.codename);
        }
    }
}
