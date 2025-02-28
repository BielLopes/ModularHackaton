use std::{
    error::Error,
    fs::{self, File},
    io::BufReader,
    net::IpAddr,
};

use data_encoding::BASE32_NOPAD_NOCASE as BASE32;
use if_addrs::get_if_addrs;
use iroh::{protocol::Router, Endpoint};
use iroh_blobs::{
    net_protocol::Blobs,
    rpc::client::blobs::{MemClient, WrapOption},
    util::SetTagOption,
};
use serde_json::Value;
use swarm_discovery::Discoverer;
use tokio::runtime::{Builder, Runtime};
use walkdir::WalkDir;

use crate::{
    config::{Configuer, Repo},
    init::Hasher,
    share::compression::Compression,
    PORT,
};

/// Class to share repository data
pub struct ShareSeed {
    repo: Repo,
    peer_id: Option<String>,
    router: Option<Router>,
    runtime: Option<Runtime>,
    addrs: Option<Vec<IpAddr>>,
}

impl ShareSeed {
    pub fn new(repo: Repo) -> Self {
        Self {
            repo,
            peer_id: None,
            router: None,
            runtime: None,
            addrs: None,
        }
    }

    // TODO: spawn thread to share repository
    // NOW: creating chanel to every file and locking program
    pub async fn share(&mut self) -> Result<(), Box<dyn Error>> {
        // Config iroh network to share repositories
        let endpoint = Endpoint::builder().discovery_n0().bind().await?;
        let blobs = Blobs::memory().build(&endpoint);
        let router = Router::builder(endpoint)
            .accept(iroh_blobs::ALPN, blobs.clone())
            .spawn()
            .await?;
        let node_id = router.endpoint().node_id();
        println!("Sharing from node_id: {}", node_id.to_string());
        let local_peer_id = BASE32.encode(node_id.as_bytes()).to_lowercase();
        let blobs = blobs.client();
        let rt = Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("build runtime");
        let addrs = get_if_addrs()
            .unwrap()
            .into_iter()
            .map(|i| i.addr.ip())
            .collect::<Vec<_>>();

        self.peer_id = Some(local_peer_id);
        self.router = Some(router);
        self.runtime = Some(rt);
        self.addrs = Some(addrs);

        self.copy_repo(&blobs).await?;
        println!("[INFO] Repository files copied! Starting to share...");
        self.share_repo(&blobs).await?;

        Ok(())
    }

    /// Put all the repository files on the iroh network
    async fn share_repo(&self, client: &MemClient) -> Result<(), Box<dyn Error>> {
        let dest_folder = Configuer::find_gitfreedom_config()?
            .1
            .join(self.repo.full_name());

        for entry in WalkDir::new(dest_folder).into_iter().filter_map(Result::ok) {
            let path = entry.path().to_path_buf();

            let _blob = client
                .add_from_path(path.clone(), true, SetTagOption::Auto, WrapOption::NoWrap)
                .await?
                .finish()
                .await?;
        }
        let local_peer_id = self.peer_id.as_ref().unwrap().clone();
        let _guard = Discoverer::new(self.repo.full_name(), local_peer_id.clone())
            .with_addrs(*PORT, self.addrs.as_ref().unwrap().clone())
            .with_callback(move |peer_id, peer| {
                if peer_id != local_peer_id {
                    println!("Discovered peer {peer_id} at {:?}", peer);
                }
            })
            .spawn(self.runtime.as_ref().unwrap().handle())
            .expect("Discoverer spawn error!");

        tokio::signal::ctrl_c().await?;
        Ok(())
    }

    /// Copy repository to a temporary folder to copy without the concern of modifications.
    // TODO: Return just the UpdateRepositoryMetadata error
    async fn copy_repo(&self, client: &MemClient) -> Result<(), Box<dyn Error>> {
        let dest_folder = Configuer::find_gitfreedom_config()?
            .1
            .join(self.repo.full_name());

        // Delete the destination folder if exists
        if dest_folder.exists() {
            fs::remove_dir_all(&dest_folder)?;
        }
        // Garantee that the destination folder exists
        fs::create_dir_all(&dest_folder)?;

        // READ {GF_JSON} file and get hash and data
        let gf_hash = Hasher::get_manifest_hash(&self.repo.path, client).await;
        let file = File::open(&self.repo.path)?;
        let reader = BufReader::new(file);
        let json_data: Value = serde_json::from_reader(reader)?;

        // Take the repository manifest and save it compressed on temporary folder
        let dest_path = dest_folder.join(gf_hash);
        Compression::compress(&self.repo.path, &dest_path)?;

        // Obtém a pasta .git
        if let Some(parent) = self.repo.path.parent() {
            let git_folder = parent.join(".git");
            if git_folder.exists() && git_folder.is_dir() {
                if let Some(files) = json_data.get("files").and_then(|f| f.as_object()) {
                    for (hash, file_info) in files {
                        if let Some(path) = file_info.get("path") {
                            if let Some(path_str) = path.as_str() {
                                let source_path = git_folder.join(path_str);
                                let dest_path = dest_folder.join(hash);

                                // Save the file compressed on temporary folder
                                if source_path.exists() {
                                    Compression::compress(&source_path, &dest_path)?;
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
