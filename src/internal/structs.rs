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
        let a: Sorted = Sorted { repo, aur, nf };
        a
    }
}

#[derive(Clone, Copy)]
pub struct Options {
    pub verbosity: i32,
    pub noconfirm: bool,
    pub asdeps: bool,
}
