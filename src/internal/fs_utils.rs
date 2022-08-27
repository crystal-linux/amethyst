use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
};

use futures::future;
use tokio::fs;

#[tracing::instrument(level = "debug")]
pub async fn rmdir_recursive(path: &Path) -> std::io::Result<()> {
    let mut files: Vec<PathBuf> = Vec::new();
    let mut folders: Vec<PathBuf> = Vec::new();

    if path.is_dir() {
        folders.push(path.into());
    } else {
        files.push(path.into());
    }

    let mut folders_to_scan: VecDeque<_> = folders.clone().into();

    while let Some(path) = folders_to_scan.pop_front() {
        let mut dir_content = fs::read_dir(&path).await?;

        while let Some(entry) = dir_content.next_entry().await? {
            let entry = entry.path();

            if entry.is_dir() {
                folders_to_scan.push_back(entry.clone());
                folders.push(entry);
            } else {
                files.push(entry);
            }
        }
    }

    tracing::debug!("Deleting {} files", files.len());
    future::try_join_all(files.into_iter().map(fs::remove_file)).await?;

    tracing::debug!("Deleting {} folders", folders.len());

    folders.reverse();
    for folder in folders {
        tracing::trace!("Deleting {folder:?}");
        fs::remove_dir(folder).await?;
    }

    Ok(())
}
