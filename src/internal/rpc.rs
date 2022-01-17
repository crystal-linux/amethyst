#![allow(dead_code)]

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

pub struct InfoResults {
    pub found: bool,
    pub package: Option<Package>
}

pub fn rpcinfo(pkg: String) -> InfoResults {
    let res = reqwest::blocking::get(&format!(
        "https://aur.archlinux.org/rpc/?v=5&type=info&arg={}",
        pkg
    )).unwrap().json::<SearchResults>().unwrap();

    if res.results.is_empty() {
        InfoResults {
            found: false,
            package: None
        }
    } else {
        InfoResults {
            found: true,
            package: Some(res.results[0].clone())
        }
    }
}

pub fn rpcsearch(pkg: String) -> SearchResults {
    reqwest::blocking::get(&format!(
        "https://aur.archlinux.org/rpc/?v=5&type=search&arg={}",
        pkg
    )).unwrap().json().unwrap()
}
