pub mod clone;
pub mod config;
pub mod errors;
pub mod init;
pub mod share;

// Files and folder names
pub static CONFIG_FOLDER: &str = ".gitfreedom";
pub static REPOS: &str = "repos.json";
pub static PUBLIC_KEY_FILE: &str = "PUBLIC_KEY";
pub static GF_JSON: &str = "gitfreedom.json";
pub static NAME_PARAM: &str = "gitfreedom_repo_name";
pub static FILES_JSON_OBJECT_NAME: &str = "files";

// Network configs
pub static PORT: &u16 = &7777;
pub static RPC_URL: &str = "http://localhost:8547";
pub static STYLUS_CONTRACT_ADDRESS: &str = "0xa6e41ffd769491a42a6e5ce453259b93983a22ef";
pub static PRIVATE_KEY: &str = "b6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659";
