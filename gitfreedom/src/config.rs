pub mod configuer;
pub mod key_handler;
pub mod repo;

use std::{error::Error, fs::File, io::Write};

use crate::{errors::Errors, PUBLIC_KEY_FILE, REPOS};
pub use configuer::Configuer;
pub use key_handler::Key;
pub use repo::Repo;

pub fn run(pub_key: Option<String>) -> Result<(), Box<dyn Error>> {
    // Verify if the public key is valid
    if pub_key.is_some() {
        if !Key::new(pub_key.clone().unwrap()).is_valid_public_key() {
            return Err(Box::new(Errors::InvalidPublicKey));
        }
    }

    // Check if the config folder exists and is a directory.
    let config_folder = Configuer::find_gitfreedom_config()?.1;
    if !config_folder.exists() {
        std::fs::create_dir(&config_folder)?;
        println!(
            "[INFO] Created Git Freedom folder: {}",
            config_folder.to_string_lossy()
        );
    } else if config_folder.is_file() {
        return Err(Box::new(Errors::ConfigFolderIsFile));
    }

    // Write the key to the {PUBLIC_KEY_FILE} file, If the key is None, delete the file.
    let key_path = config_folder.join(PUBLIC_KEY_FILE);
    if pub_key.is_some() {
        let mut pb_key_file = File::create(&key_path)?;
        pb_key_file.write_all(pub_key.unwrap().as_bytes())?;
        println!(
            "[INFO] Created {PUBLIC_KEY_FILE} file at {}",
            key_path.to_string_lossy().to_string()
        );
    } else if key_path.exists() {
        std::fs::remove_file(&key_path)?;
        println!(
            "[INFO] Deleted {PUBLIC_KEY_FILE} file at {}",
            key_path.to_string_lossy()
        );
    }

    // Config the repositories list
    let repos_path = config_folder.join(REPOS);
    if !repos_path.exists() {
        let mut repos = File::create(&repos_path)?;
        repos.write_all(
            serde_json::Value::Object(serde_json::Map::new())
                .to_string()
                .as_bytes(),
        )?;
        println!(
            "[INFO] Created {REPOS} file at {}",
            repos_path.to_string_lossy()
        );
    } else {
        println!("[INFO] Repositories list file already exists");
    }

    Ok(())
}
