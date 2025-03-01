use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;

use ethers::{
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::Address,
};

use crate::config::{Key, Repo};
use crate::{PRIVATE_KEY, RPC_URL, STYLUS_CONTRACT_ADDRESS};

type RepoData = (String, String, String, Vec<Address>);

pub struct RepositoryContract {
    key: Key,
    repo: Option<Repo>,
    name: String,
}

impl RepositoryContract {
    pub fn new(key: Key, repo: Option<Repo>, name: String) -> Self {
        Self { key, repo, name }
    }

    pub async fn get_hash_manifest(&self) -> Result<String, Box<dyn Error>> {
        abigen!(
            Contract,
            r#"[
                function getRepository(string calldata name, address owner) external view returns (string memory, string memory, string memory, address[] memory)
            ]"#
        );

        let provider = Provider::<Http>::try_from(RPC_URL)?;
        let address: Address = STYLUS_CONTRACT_ADDRESS.parse()?;

        let privkey = String::from(PRIVATE_KEY);
        let wallet = LocalWallet::from_str(&privkey)?;
        let chain_id = provider.get_chainid().await?.as_u64();
        let client = Arc::new(SignerMiddleware::new(
            provider,
            wallet.clone().with_chain_id(chain_id),
        ));

        // Get the contract instance
        let contract = Contract::new(address, client.clone());

        // Get the repository data
        // let owner_address: Address = self.key.key.parse()?;
        let repo: RepoData = contract
            .get_repository(self.name.clone(), self.key.get_key().parse()?)
            .call()
            .await
            .expect("Could not get repository data!");

        println!("Repository:  {:?}", repo);

        Ok(repo.1)
    }

    pub async fn deploy(&self) -> Result<bool, Box<dyn Error>> {
        println!(
            "[INFO] Deploying repository {} on chain...",
            self.repo.clone().unwrap().full_name()
        );
        abigen!(
            Contract,
            r#"[
                function addRepository(string calldata name, string calldata description, string calldata hash) external;
            ]"#
        );

        let provider = Provider::<Http>::try_from(RPC_URL)?;
        let address: Address = STYLUS_CONTRACT_ADDRESS.parse()?;

        let privkey = self.key.get_key();
        let wallet = LocalWallet::from_str(&privkey)?;
        let chain_id = provider.get_chainid().await?.as_u64();
        let client = Arc::new(SignerMiddleware::new(
            provider,
            wallet.clone().with_chain_id(chain_id),
        ));

        // Get the contract instance
        let contract = Contract::new(address, client.clone());

        // Deploy the repository
        let pending = contract.add_repository(
            self.name.clone(),
            String::from("Description Test."),
            self.repo.clone().unwrap().hash,
        );
        if let Some(receipt) = pending.send().await?.await? {
            println!("Receipt = {:?}", receipt);
        }

        Ok(true)
    }
}
