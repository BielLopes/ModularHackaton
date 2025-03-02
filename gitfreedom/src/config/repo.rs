use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Status of the repository.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RepoStatus {
    Initialized,
    Seeding(SeedStatus),
    Error,
}

/// Status of the seeding process.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SeedStatus {
    Updated,
    Outdated,
    Starting,
}

/// Representation of a repository in the {REPOS} file.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Repo {
    pub name: String,
    pb_key: String,
    pub hash: String,
    pub path: PathBuf,
    last_commit: Option<String>,
    last_modification_datatime: Option<DateTime<Utc>>,
    status: Option<RepoStatus>,
}

impl Repo {
    pub fn new(
        name: String,
        pb_key: String,
        hash: String,
        path: PathBuf,
        last_commit: Option<String>,
        last_modification_datatime: Option<DateTime<Utc>>,
        status: Option<RepoStatus>,
    ) -> Self {
        Self {
            name,
            pb_key,
            hash,
            path,
            last_commit,
            last_modification_datatime,
            status,
        }
    }

    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    pub fn full_name(&self) -> String {
        format!("{}-{}", self.name, self.pb_key)
    }

    pub fn split_name(full_name: &String) -> (String, String) {
        let name: Vec<&str> = full_name.rsplitn(2, '-').collect();
        (
            name.get(0).unwrap().to_string(),
            name.get(1).unwrap().to_string(),
        )
    }
}
