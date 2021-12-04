#[derive(serde::Deserialize, Debug, Clone)]
pub struct Package {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(rename = "Depends")]
    pub depends: Vec<String>,
    #[serde(default)]
    #[serde(rename = "MakeDepends")]
    pub make_depends: Vec<String>,
}

#[derive(serde::Deserialize)]
pub struct SearchResults {
    pub resultcount: u32,
    pub results: Vec<Package>,
}

pub fn rpcinfo(pkg: &str) -> Package {
    let res = reqwest::blocking::get(&format!(
        "https://aur.archlinux.org/rpc/?v=5&type=info&arg={}",
        pkg
    )).unwrap();

    res.json::<SearchResults>().unwrap().results[0].clone()
}

pub fn rpcsearch(pkg: &str) -> SearchResults {
    let res = reqwest::blocking::get(&format!(
        "https://aur.archlinux.org/rpc/?v=5&type=search&arg={}",
        pkg
    )).unwrap();

    res.json().unwrap()
}
