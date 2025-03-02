mod clone_seed;

use std::{env, error::Error};

use chrono::Utc;

use crate::{
    config::{
        repo::{RepoStatus, SeedStatus},
        Configuer, Key, Repo,
    },
    errors::Errors,
    git::Git,
    share::RepositoryContract,
    REPOS,
};
pub use clone_seed::CloneSeed;

pub async fn run(full_name: String) -> Result<(), Box<dyn Error>> {
    // Get the repository name and public key
    let (pb_key, name) = Repo::split_name(&full_name);

    // Validate the public key
    let pb_key = Key::new(pb_key);
    if !pb_key.is_valid_public_key() {
        return Err(Box::new(Errors::InvalidPublicKey));
    }

    // Retrive manifest hash
    let contract = RepositoryContract::new(pb_key.clone(), None, name.clone());
    let hash = contract.get_hash_manifest().await?;

    // Clone the repository
    let mut seed = CloneSeed::new(full_name, hash.clone());
    seed.clone().await?;
    println!("[INFO] Repository cloned!");

    // Restore the repository files
    let path = env::current_dir()?.canonicalize()?.join(&name);
    let git = Git::new(path.clone());
    git.restore_workdir()?;

    // Create and add repo to {REPOS} file
    let congiguer = Configuer::new();
    let repo = Repo::new(
        name,
        pb_key.get_key(),
        hash,
        path,
        Some(git.get_last_commit()?),
        Some(Utc::now()),
        Some(RepoStatus::Seeding(SeedStatus::Starting)),
    );
    congiguer.add_repo(repo).map_err(|error| {
        if let Some(err) = error.downcast_ref::<Errors>() {
            match err {
                Errors::RepoNameAlreadyExists => {
                    println!("[WARN] Repository already exists in {REPOS} file.");
                }
                _ => return error,
            }
        }
        error
    })
}
