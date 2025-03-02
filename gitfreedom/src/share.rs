pub mod compression;
mod contract;
mod share_seed;

use std::{env, error::Error, path::PathBuf};

use crate::{
    config::{Configuer, Key},
    errors::Errors,
    git::Git,
};
pub use compression::Compression;
pub use contract::RepositoryContract;
use share_seed::ShareSeed;

pub async fn run(pv_key: PathBuf, name: Option<String>) -> Result<(), Box<dyn Error>> {
    // Check private key
    let key = Key::new_from_file(pv_key);
    if !key.is_valid_private_key() {
        return Err(Box::new(Errors::InvalidPrivateKey));
    }

    // Get reository name
    let configuer = Configuer::new();
    let path = env::current_dir()?.canonicalize()?;
    let mut git = Git::new(path);
    let name = match name {
        Some(name) => configuer.full_name(name)?,
        None => configuer.full_name(git.get_local_repo(None)?.0)?,
    };

    // Get the repository metadata and check existence
    let repo = configuer.get_repo(name)?;
    if !repo.exists() {
        return Err(Box::new(Errors::GitFreedomRepositoryNotFount));
    }

    // Deploy repository hash info on Blockchain
    let contract = RepositoryContract::new(key, Some(repo.clone()), repo.name.clone());
    if contract.deploy().await? {
        println!("[INFO] Succefully deploy repository on chain!");
    } else {
        return Err(Box::new(Errors::RepositoryDeployFail));
    }

    // Start to share the repository data
    let mut seed = ShareSeed::new(repo);
    seed.share().await.expect("Error sharing repository!");

    Ok(())
}
