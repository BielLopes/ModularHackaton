use thiserror::Error;

use crate::REPOS;

/// Custom errors for the gitfreedom CLI
#[derive(Error, Debug)]
pub enum Errors {
    #[error("Git Freedom folder not found. Please run `gf config` to initialize Git Freedom.")]
    GitFreedomFolderNotFound,
    #[error("Config folder must be a directory! Delete the file.")]
    ConfigFolderIsFile,
    #[error("Can't open {REPOS} file. Please run 'gf config' first.")]
    ReposFileNotFound,
    #[error("Culd not generate a hash for the manifest file.")]
    ManifestHashError,
    #[error("Repository name already used.")]
    RepoNameAlreadyExists,
    #[error("Public key not found. Please run 'gf config --public-key <PUBLIC_KEY>' first.")]
    PublicKeyNotFound,
    #[error("Private key not found. Please verify path.")]
    PrivateKeyNotFound,
    #[error("Can't find Git Freedom file on repository! Please run `gf init` first.")]
    GitFreedomRepositoryNotFount,
    #[error("Can't find .git folderA")]
    GitFolderNotFound,
    #[error("Should have a branch named 'main'.")]
    MainBranchNotFound,
    #[error("Invalid public key.")]
    InvalidPublicKey,
    #[error("Invalid private key.")]
    InvalidPrivateKey,
    #[error("Fail to deploy repository on chain!")]
    RepositoryDeployFail,
    #[error("Can't file {0} on .git folder. Consider run `gf update` first")]
    UpdateRepositoryMetadata(#[from] std::io::Error),
}
