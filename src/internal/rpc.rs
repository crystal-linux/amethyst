use aur_rpc::{PackageInfo, PackageMetadata, SearchField};

use super::error::AppResult;
pub const URL: &str = "https://aur.archlinux.org/";

pub async fn rpcinfo(pkg: &str) -> AppResult<Option<PackageInfo>> {
    let packages = aur_rpc::info(vec![pkg]).await?;

    Ok(packages.into_iter().next())
}

pub async fn rpcinfo_many(pkgs: &[String]) -> AppResult<Vec<PackageInfo>> {
    let mut futures = vec![];
    for pkg in pkgs {
        futures.push(aur_rpc::info(vec![pkg]));
    }

    let mut results = vec![];
    for future in futures {
        let mut result = future.await?;
        results.append(&mut result);
    }

    Ok(results)
}

pub async fn rpcsearch(
    query: String,
    by_field: Option<SearchField>,
) -> AppResult<Vec<PackageMetadata>> {
    let search_results = if let Some(field) = by_field {
        aur_rpc::search_by(field, query).await?
    } else {
        aur_rpc::search(query).await?
    };

    Ok(search_results)
}
