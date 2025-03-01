use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::{self, File},
    io::BufReader,
    net::IpAddr,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use data_encoding::BASE32_NOPAD_NOCASE as BASE32;
use if_addrs::get_if_addrs;
use iroh::{protocol::Router, Endpoint, NodeAddr, NodeId};
use iroh_blobs::{
    net_protocol::Blobs,
    rpc::client::blobs::{MemClient, ReadAtLen},
};
use swarm_discovery::{Discoverer, Peer};
use tokio::runtime::{Builder, Runtime};

use crate::{config::Repo, share::Compression, GF_JSON, PORT};

// type PeerMap = Arc<Mutex<HashMap<String, Vec<NodeAddr>>>>;

/// Class to share repository data
pub struct CloneSeed {
    full_name: String,
    manifest: String,
    peer_id: Option<String>,
    router: Option<Router>,
    runtime: Option<Runtime>,
    addrs: Option<Vec<IpAddr>>,
    peer_map: Arc<Mutex<HashMap<String, Vec<NodeAddr>>>>,
}

impl CloneSeed {
    pub fn new(full_name: String, manifest_hash: String) -> Self {
        Self {
            full_name,
            manifest: manifest_hash,
            peer_id: None,
            router: None,
            runtime: None,
            addrs: None,
            peer_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn clone(&mut self) -> Result<(), Box<dyn Error>> {
        // Config iroh network to share repositories
        let endpoint = Endpoint::builder().discovery_n0().bind().await?;
        let blobs = Blobs::memory().build(&endpoint);
        let router = Router::builder(endpoint)
            .accept(iroh_blobs::ALPN, blobs.clone())
            .spawn()
            .await?;
        let node_id = router.endpoint().node_id();
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

        self.clone_repo(&blobs).await?;
        println!("[INFO] Repository cloned!");

        Ok(())
    }

    async fn get_manifest(&self, client: &MemClient) -> Result<PathBuf, Box<dyn Error>> {
        let channel_name = self.full_name.clone();
        let local_peer_id = format!(
            "gf_peer_{}",
            self.router
                .as_ref()
                .unwrap()
                .endpoint()
                .node_id()
                .to_string()
                .chars()
                .take(10)
                .collect::<String>(),
        );

        let _guard = Discoverer::new(channel_name, local_peer_id.clone())
            .with_addrs(*PORT, self.addrs.as_ref().unwrap().clone())
            .with_callback(Self::handle_discovered_peers(
                local_peer_id,
                Arc::clone(&self.peer_map),
                self.manifest.clone(),
            ))
            .spawn(self.runtime.as_ref().unwrap().handle())
            .expect("Discoverer spawn");

        println!("Looking for peers with repository {}.", self.full_name);
        tokio::time::sleep(std::time::Duration::from_secs(20)).await;

        let peers = {
            let map = self.peer_map.lock().unwrap();
            map.get(&self.manifest).cloned()
        };

        if let Some(peers) = peers {
            for peer in peers {
                if let Ok(_) = client
                    .download(self.manifest.parse().expect("Error on parse"), peer.clone())
                    .await
                    .expect("Error on download!")
                    .finish()
                    .await
                {
                    break;
                }
            }
        } else {
            println!("No peers found for file {}.", self.manifest);
        }

        // Crete the repository folder just with the repository name
        let path = env::current_dir()?.canonicalize()?;
        let path = path.join(Repo::split_name(&self.full_name).1);
        fs::create_dir_all(&path)?;

        // Save the repository manifest
        let path = path.join(GF_JSON);
        let mut file = tokio::fs::File::create(&path).await?;
        let mut reader = client
            .read_at(self.manifest.parse()?, 0, ReadAtLen::All)
            .await?;
        tokio::io::copy(&mut reader, &mut file).await?;

        // Decompress the file
        Compression::decompress(&path)?;

        Ok(path)
    }

    fn handle_discovered_peers(
        local_peer_id: String,
        peer_map: Arc<Mutex<HashMap<String, Vec<NodeAddr>>>>,
        blob_hash: String,
    ) -> impl FnMut(&str, &Peer) {
        move |peer_id, _peer| {
            // Ignore myself as a peer
            if peer_id != local_peer_id {
                // Decode the peer id to get his node id
                let decoded_node_id = BASE32
                    .decode(peer_id.as_bytes())
                    .expect("Failed to recover with base 32!");
                let mut fixed_array = [0u8; 32];
                let len = decoded_node_id.len().min(32);
                fixed_array[..len].copy_from_slice(&decoded_node_id[..len]);

                // Add the peer to the peer map
                if let Ok(node_id) = NodeId::from_bytes(&fixed_array) {
                    let mut map = peer_map.lock().unwrap();
                    map.entry(blob_hash.clone())
                        .or_insert_with(Vec::new)
                        .push(NodeAddr::from(node_id));
                }
            }
        }
    }

    async fn clone_repo(&self, client: &MemClient) -> Result<(), Box<dyn Error>> {
        // Get the repository manifest
        let path = self.get_manifest(client).await?;
        println!("[INFO] Repository manifest retreived!");
        println!("[INFO] Starting to clone repository...");

        // Read the repository manifest as json

        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let json_data: serde_json::Value = serde_json::from_reader(reader)?;

        // Create the .git folder
        let parent = path.parent().unwrap();
        let git_folder = parent.join(".git");
        fs::create_dir_all(&git_folder)?;

        // Iterate over each file in the manifest and download it
        if let Some(files) = json_data.get("files").and_then(|f| f.as_object()) {
            for (hash, file_info) in files {
                if let Some(path) = file_info.get("path") {
                    if let Some(path_str) = path.as_str() {
                        // Ensure the file folder exists
                        let source_path = git_folder.join(path_str);
                        fs::create_dir_all(&source_path.parent().unwrap())?;

                        let peers = {
                            let map = self.peer_map.lock().unwrap();
                            map.get(&self.manifest).cloned()
                        }
                        .unwrap();

                        // Chose the first peer that are on the chanel
                        for peer in peers {
                            if let Ok(_) = client
                                .download(hash.parse()?, peer.clone())
                                .await?
                                .finish()
                                .await
                            {
                                break;
                            }
                        }

                        // Write the file
                        let mut file = tokio::fs::File::create(&source_path).await?;
                        let mut reader = client.read_at(hash.parse()?, 0, ReadAtLen::All).await?;
                        tokio::io::copy(&mut reader, &mut file).await?;

                        // Decompress the file
                        Compression::decompress(&source_path)?;
                    }
                }
            }
        }

        Ok(())
    }
}
