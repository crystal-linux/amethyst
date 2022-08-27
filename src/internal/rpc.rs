use aur_rpc::{PackageInfo, PackageMetadata};

use super::error::AppResult;
pub const URL: &str = "https://aur.archlinux.org/";

pub async fn rpcinfo(pkg: &str) -> AppResult<Option<PackageInfo>> {
    let packages = aur_rpc::info(vec![pkg]).await?;

    Ok(packages.into_iter().next())
}

pub async fn rpcsearch(pkg: String) -> AppResult<Vec<PackageMetadata>> {
    let search_results = aur_rpc::search(pkg).await?;

    Ok(search_results)
}
