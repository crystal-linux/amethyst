#[derive(Debug, serde::Serialize)]
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

#[derive(Clone, Copy)]
pub struct Options {
    pub verbosity: usize,
    pub noconfirm: bool,
    pub asdeps: bool,
    pub toplevel: bool,
}
