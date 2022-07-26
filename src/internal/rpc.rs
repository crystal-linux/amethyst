use std::sync::Arc;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Package {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Depends")]
    #[serde(default)]
    pub depends: Vec<String>,
    #[serde(rename = "MakeDepends")]
    #[serde(default)]
    pub make_depends: Vec<String>,
}

#[derive(serde::Deserialize)]
pub struct SearchResults {
    pub resultcount: u32,
    pub results: Vec<Package>,
}

#[derive(Clone)]
pub struct InfoResults {
    pub found: bool,
    pub package: Option<Package>,
}

pub const URL: &str = "https://aur.archlinux.org/";

pub fn rpcinfo(pkg: String) -> InfoResults {
    // Initialise TLS connector
    let tls_connector = Arc::new(native_tls::TlsConnector::new().unwrap());

    // Build request agent
    let agent = ureq::AgentBuilder::new()
        .tls_connector(tls_connector)
        .build();

    // Send request and parse results into json
    let res: SearchResults = agent
        .get(&format!(
            "https://aur.archlinux.org/rpc/?v=5&type=info&arg={}",
            pkg
        ))
        .call()
        .unwrap()
        .into_json()
        .unwrap();

    // Check if package was found
    if res.results.is_empty() {
        InfoResults {
            found: false,
            package: None,
        }
    } else {
        InfoResults {
            found: true,
            package: Some(res.results[0].clone()),
        }
    }
}

pub fn rpcsearch(pkg: String) -> SearchResults {
    // Initialise TLS connector
    let tls_connector = Arc::new(native_tls::TlsConnector::new().unwrap());

    // Build request agent
    let agent = ureq::AgentBuilder::new()
        .tls_connector(tls_connector)
        .build();

    // Send request and parse results into json
    agent
        .get(&format!(
            "https://aur.archlinux.org/rpc/?v=5&type=search&arg={}",
            pkg
        ))
        .call()
        .unwrap()
        .into_json::<SearchResults>()
        .unwrap()
}
