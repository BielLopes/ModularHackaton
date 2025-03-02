mod file_node;
mod hasher;
mod scaner;

use std::{env, error::Error, fs::File, io::Write};

use chrono::Utc;
use iroh::Endpoint;
use iroh_blobs::net_protocol::Blobs;
use serde_json::Value;

use crate::{
    config::{
        repo::{Repo, RepoStatus},
        Configuer,
    },
    git::Git,
    FILES_JSON_OBJECT_NAME, GF_JSON, NAME_PARAM,
};
pub use file_node::FileNode;
pub use hasher::Hasher;
use scaner::Scaner;

pub async fn run(name: Option<String>) -> Result<(), Box<dyn Error>> {
    // Get the repository name and path
    let path = env::current_dir()?.canonicalize()?;
    let mut git = Git::new(path);
    let (name, path) = git.get_local_repo(name)?;

    // Verify if the configs are initialized
    let configuer = Configuer::new();
    let name = configuer.full_name(name)?;
    configuer.check_uniqueness(name.clone())?;

    // Initialize endpoint to create a client
    let endpoint = Endpoint::builder().discovery_n0().bind().await.unwrap();
    let blobs = Blobs::memory().build(&endpoint);
    let client = blobs.client();

    // Add {GF_JSON} to .gitignore if not already present
    git.ignore_metadata_file()?;

    // Get the directory tree and add the repository name and public key to the JSON tree
    let scaner = Scaner::new(path.clone().join(".git"), &client);
    let json_files = scaner.get_files().await;
    let mut repo_manifest = Value::default();
    repo_manifest[NAME_PARAM] = Value::String(name.clone());
    repo_manifest[FILES_JSON_OBJECT_NAME] = json_files;

    // Write the JSON tree to the {GF_JSON} file
    let gf_path = path.join(GF_JSON);
    let mut gf_json = File::create(&gf_path)?;
    gf_json.write_all(serde_json::to_string_pretty(&repo_manifest)?.as_bytes())?;
    println!("[INFO] Created {GF_JSON} file.");

    // Get the hashe of the blobs in the {GF_JSON} file
    let gf_hash = Hasher::get_manifest_hash(&gf_path, &client).await;

    // Get the repository name and public key
    let (pb_key, name) = Repo::split_name(&name);

    // Cteate de Repo struct
    let repo_params = Repo::new(
        name,
        pb_key,
        gf_hash,
        gf_path,
        Some(git.get_last_commit()?),
        Some(Utc::now()),
        Some(RepoStatus::Initialized),
    );

    // Add the repository to the {REPOS} file
    configuer.add_repo(repo_params)?;

    Ok(())
}
