use std::path::PathBuf;

use iroh_blobs::{
    rpc::client::blobs::{MemClient, WrapOption},
    util::SetTagOption,
};
use tempfile::NamedTempFile;

use crate::share::Compression;

pub struct Hasher;

impl Hasher {
    pub async fn get_manifest_hash(path: &PathBuf, client: &MemClient) -> String {
        // Create a temporary file for the compressed archive
        let temp_file = NamedTempFile::new().expect("Failed to create temporary file");
        let temp_path = temp_file.path().to_path_buf();

        // Compress the file into the temporary archive
        Compression::compress(path, &temp_path).expect("Failed to compress file");
        // create the blobs, store them on a local memory and get the manifest hash
        let manifest = client
            .add_from_path(temp_path, true, SetTagOption::Auto, WrapOption::NoWrap)
            .await
            .expect("Failed to add blob to local memory.")
            .finish()
            .await
            .expect("Failed to add blob to local memory.");

        // delete the blob from memory
        client
            .delete_blob(manifest.hash)
            .await
            .expect("Failed to delete blob");

        manifest.hash.to_string()
    }
}
