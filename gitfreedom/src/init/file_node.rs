use std::path::PathBuf;

use iroh_blobs::rpc::client::blobs::MemClient;
use serde::{Deserialize, Serialize};

use super::hasher::Hasher;

/// Representation of a directory tree node.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileNode {
    path: PathBuf,
}

impl FileNode {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub async fn get_hash(&self, client: &MemClient) -> String {
        Hasher::get_manifest_hash(&self.path, &client).await
    }

    pub fn simplify_path(&mut self) {
        let mut path_str = self.path.to_str().unwrap().to_string();
        path_str = path_str
            .rfind(".git/")
            .map(|idx| path_str[idx + 5..].to_string())
            .unwrap_or(String::default());
        self.path = PathBuf::from(path_str);
    }
}
