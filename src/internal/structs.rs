#[derive(Debug)]
pub struct Sorted {
    #[allow(dead_code)]
    repo: Vec<String>,
    #[allow(dead_code)]
    aur: Vec<String>,
    #[allow(dead_code)]
    nf: Vec<String>
}

impl Sorted {
    pub fn new(repo: Vec<String>, aur: Vec<String>, nf: Vec<String>) -> Self {
        let a: Sorted = Sorted {
            repo,
            aur,
            nf
        };
        a
    }
}