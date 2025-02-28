mod file_node;
mod hasher;
mod scaner;

use std::env;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;

use chrono::Utc;
use git2::{BranchType, Repository};
use iroh::Endpoint;
use iroh_blobs::net_protocol::Blobs;
use serde_json::Value;

use crate::config::{
    repo::{Repo, RepoStatus},
    Configuer,
};
use crate::errors::Errors;
use crate::{FILES_JSON_OBJECT_NAME, GF_JSON, NAME_PARAM};
pub use file_node::FileNode;
pub use hasher::Hasher;
use scaner::Scaner;

pub async fn run(name: Option<String>) -> Result<(), Box<dyn Error>> {
    // Get the repository name and path
    let (name, path) = get_local_repo(name)?;

    // Verify if the configs are initialized
    let configuer = Configuer::new();
    let name = configuer.full_name(name)?;
    configuer.verify_init(name.clone())?;

    // Verify if the current directory is a git repository
    let repo = Repository::open(&path)?;
    let git_path = repo.path().to_path_buf();

    // Initialize endpoint to create a client
    let endpoint = Endpoint::builder().discovery_n0().bind().await.unwrap();
    let blobs = Blobs::memory().build(&endpoint);
    let client = blobs.client();

    // Get the directory tree and add the repository name and public key to the JSON tree
    let scaner = Scaner::new(git_path, &client);
    let json_files = scaner.get_files().await;
    let mut repo_manifest = Value::default();
    repo_manifest[NAME_PARAM] = Value::String(name.clone());
    repo_manifest[FILES_JSON_OBJECT_NAME] = json_files;

    // Write the JSON tree to the {GF_JSON} file
    let gf_path = path.join(GF_JSON);
    let mut gf_json = File::create(&gf_path)?;
    gf_json.write_all(serde_json::to_string_pretty(&repo_manifest)?.as_bytes())?;
    println!("[INFO] Created {GF_JSON} file.");

    // Add {GF_JSON} to .gitignore if not already present
    ignore_config(path)?;

    // Get the hashe of the blobs in the {GF_JSON} file
    let gf_hash = Hasher::get_manifest_hash(&gf_path, &client).await;

    // Get the last commit hash
    let branch = repo
        .find_branch("main", BranchType::Local)
        .expect(Errors::MainBranchNotFound.to_string().as_str());
    let last_commit = branch.get().peel_to_commit()?.id().to_string();

    // Get the repository name and public key
    let name: Vec<&str> = name.splitn(2, '-').collect();
    let (name, pb_key) = (
        name.get(0).unwrap().to_string(),
        name.get(1).unwrap().to_string(),
    );

    // Cteate de Repo struct
    let repo_params = Repo::new(
        name,
        pb_key,
        gf_hash,
        gf_path,
        Some(last_commit),
        Some(Utc::now()),
        Some(RepoStatus::Initialized),
    );

    // Add the repository to the {REPOS} file
    configuer.add_repo(repo_params)?;

    Ok(())
}

/// The repositry name is the name of the current directory if not provide
pub fn get_local_repo(name: Option<String>) -> Result<(String, PathBuf), Box<dyn Error>> {
    // If the current directory is a .git folder, get the root path of the repository
    let mut path = env::current_dir()?.canonicalize()?;
    if path.ends_with(".git") {
        path = path
            .parent()
            .expect("Can't find the repository root path.")
            .canonicalize()?;
    }
    let name = name.unwrap_or_else(|| path.file_name().unwrap().to_str().unwrap().to_string());

    Ok((name, path))
}

/// Ensure the {GF_JSON} are in the .gitignore file
fn ignore_config(path: PathBuf) -> Result<(), Box<dyn Error>> {
    // Add {GF_JSON} to .gitignore if not already present
    let gitignore_path = path.join(".gitignore");

    // Check if .gitignore exists and add {GF_JSON} if not already present
    if gitignore_path.exists() {
        let mut gitignore_content = String::new();
        let mut file = File::open(&gitignore_path)?;
        file.read_to_string(&mut gitignore_content)?;

        if !gitignore_content.lines().any(|line| line.trim() == GF_JSON) {
            let mut gitignore = OpenOptions::new().append(true).open(&gitignore_path)?;
            writeln!(gitignore, "{GF_JSON}")?;
            println!("[INFO] {GF_JSON} to .gitignore file.");
        } else {
            println!("[INFO] {GF_JSON} already exists in .gitignore file.");
        }
    } else {
        let mut gitignore = File::create(&gitignore_path)?;
        writeln!(gitignore, "{GF_JSON}")?;
        println!("[INFO] Created .gitignore file and added {GF_JSON}.");
    }

    Ok(())
}
