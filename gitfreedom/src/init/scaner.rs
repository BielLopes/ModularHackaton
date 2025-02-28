use std::collections::HashMap;
use std::path::PathBuf;

use iroh_blobs::rpc::client::blobs::MemClient;
use serde_json::Value;
use walkdir::WalkDir;

use super::FileNode;

/// Get the directory tree of the given path.
pub struct Scaner<'a> {
    root_path: PathBuf,
    client: &'a MemClient,
}

impl<'a> Scaner<'a> {
    pub fn new(path: PathBuf, client: &'a MemClient) -> Self {
        Self {
            root_path: path,
            client,
        }
    }

    pub async fn get_files(&self) -> Value {
        // Create the directory HashMap
        let hash_map = self.create_list(&self.root_path).await;

        Value::Object(
            hash_map
                .into_iter()
                .map(|(key, value)| (key, serde_json::to_value(value).unwrap()))
                .collect(),
        )
    }

    async fn create_list(&self, path: &PathBuf) -> HashMap<String, FileNode> {
        let mut hash_map = HashMap::new();

        for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
            if entry.path().is_file() {
                let path = entry.path().to_path_buf();
                let mut file_node = FileNode::new(path);
                let hash = file_node.get_hash(&self.client).await;
                file_node.simplify_path();
                hash_map.insert(hash, file_node);
            }
        }

        hash_map
    }
}
