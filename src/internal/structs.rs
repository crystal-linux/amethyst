#[derive(Debug, serde::Serialize)]
/// Struct for packages exiting [`crate::internal::sort()`].
pub struct Sorted {
    #[allow(dead_code)]
    pub repo: Vec<String>,
    #[allow(dead_code)]
    pub aur: Vec<String>,
    #[allow(dead_code)]
    pub nf: Vec<String>,
}

impl Sorted {
    pub fn new(repo: Vec<String>, aur: Vec<String>, nf: Vec<String>) -> Self {
        Self { repo, aur, nf }
    }
}

#[derive(Clone, Debug, Copy)]
/// Options to be passed down to internal functions
pub struct Options {
    pub noconfirm: bool,
    pub asdeps: bool,
}
