use std::{
    collections::HashMap,
    env::{self, VarError},
    error::Error,
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use super::Repo;
use crate::{errors::Errors, CONFIG_FOLDER, PUBLIC_KEY_FILE, REPOS};

pub struct Configuer {
    config_folder: PathBuf,
}

/// The logic to handle the configuration metadata files.
impl Configuer {
    pub fn new() -> Self {
        Self {
            config_folder: Configuer::find_gitfreedom_config()
                .expect("Can't find the .gitfreedom folder.")
                .1,
        }
    }

    /// Verify if the {REPOS} file exist.
    /// Return a HashMap with all the configured repositories
    fn get_repos_config(&self) -> Result<HashMap<String, Repo>, Box<dyn Error>> {
        // Try to read {REPOS} file
        let repos_path = self.config_folder.join(REPOS);
        let mut configured_repos = String::new();
        if repos_path.exists() {
            let mut repos_file = File::open(&repos_path)?;
            repos_file.read_to_string(&mut configured_repos)?;
        } else {
            return Err(Box::new(Errors::ReposFileNotFound));
        }

        // Parse the {REPOS} file
        let configured_repos: HashMap<String, Repo> =
            serde_json::from_str(&configured_repos).unwrap_or_default();

        return Ok(configured_repos);
    }

    /// Verify if the {PUBLIC_KEY_FILE} exist.
    fn get_pbkey(&self) -> Result<String, Box<dyn Error>> {
        // Try to read the public key
        let pub_key_path = self.config_folder.join(PUBLIC_KEY_FILE);
        let mut pub_key = String::new();
        if pub_key_path.exists() {
            let mut pub_key_file = File::open(&pub_key_path)?;
            pub_key_file.read_to_string(&mut pub_key)?;
        } else {
            return Err(Box::new(Errors::PublicKeyNotFound));
        }

        return Ok(pub_key);
    }

    /// Chech if the repository name is already in use.
    pub fn verify_init(&self, name: String) -> Result<(), Box<dyn Error>> {
        // Get  {REPOS} file and the public key
        let configured_repos = self.get_repos_config()?;

        // Verify if the repository name already in use
        if configured_repos.contains_key(&name) {
            return Err(Box::new(Errors::RepoNameAlreadyExists));
        }

        Ok(())
    }

    /// Get full repository name
    pub fn full_name(&self, name: String) -> Result<String, Box<dyn Error>> {
        let pub_key = self.get_pbkey()?;
        Ok(format!("{name}-{pub_key}"))
    }

    /// Add a new repository to the {REPOS} file.
    pub fn add_repo(&self, repo: Repo) -> Result<(), Box<dyn Error>> {
        let mut repos_json = self
            .get_repos_config()
            .expect("Repo info not found. Verify Config files first!");

        // Add the repository name and hash to the {REPOS} file
        repos_json.insert(repo.full_name(), repo);
        let mut repos_file = File::create(&self.config_folder.join(REPOS))?;
        repos_file.write_all(serde_json::to_string_pretty(&repos_json)?.as_bytes())?;
        println!("[INFO] Added repository configs to {REPOS} file.");

        Ok(())
    }

    /// Retrive repository metadata from its full name
    pub fn get_repo(&self, full_name: String) -> Result<Repo, Box<dyn Error>> {
        let configured_repos: HashMap<String, Repo> = self.get_repos_config()?;
        match configured_repos.get(&full_name) {
            Some(repo) => Ok(repo.clone()),
            None => Err(Box::new(Errors::GitFreedomRepositoryNotFount)),
        }
    }

    /// Find the .gitfreedom folder in the user's home directory.
    /// Returns a tuple with a boolean indicating if the config exists and the path to the folder.
    pub fn find_gitfreedom_config() -> Result<(bool, PathBuf), VarError> {
        // Get User base directory.
        let home_dir = match env::var("HOME") {
            Ok(path) => path,
            Err(_) => {
                // If the HOME variable is not defined, we try to get the USERPROFILE variable from
                // Windows.
                env::var("USERPROFILE")?
            }
        };

        // Check if the .gitfreedom folder exists in the user's home directory.
        let gitfreedom_path = Path::new(&home_dir).join(CONFIG_FOLDER);
        Ok((
            gitfreedom_path.exists() && gitfreedom_path.is_dir(),
            gitfreedom_path,
        ))
    }
}
