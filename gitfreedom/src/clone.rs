mod clone_seed;

use std::error::Error;

use crate::{config::Key, errors::Errors, share::RepositoryContract};
pub use clone_seed::CloneSeed;

pub async fn run(full_name: String) -> Result<(), Box<dyn Error>> {
    // Get the repository name and public key
    let name: Vec<&str> = full_name.splitn(2, '-').collect();
    let (name, pb_key) = (
        name.get(0).unwrap().to_string(),
        name.get(1).unwrap().to_string(),
    );

    // Validate the public key
    let pb_key = Key::new(pb_key);
    if !pb_key.is_valid_public_key() {
        return Err(Box::new(Errors::InvalidPublicKey));
    }

    // Retrive manifest hash
    let contract = RepositoryContract::new(pb_key, None, name);
    let hash = contract.get_hash_manifest().await?;

    // Clone the repository
    let mut seed = CloneSeed::new(full_name, hash);
    seed.clone().await?;

    Ok(())
}
