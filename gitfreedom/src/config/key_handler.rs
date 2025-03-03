use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use ethers::core::k256::SecretKey;
use hex::FromHex;

use crate::errors::Errors;

#[derive(Clone)]
pub struct Key {
    key: String,
}

impl Key {
    pub fn new(key: String) -> Self {
        Self { key }
    }

    pub fn new_from_file(path: PathBuf) -> Self {
        let f = File::open(path).expect(Errors::PrivateKeyNotFound.to_string().as_str());
        let mut buf_reader = BufReader::new(f);
        let mut secret = String::new();
        buf_reader
            .read_line(&mut secret)
            .expect(Errors::PrivateKeyNotFound.to_string().as_str());
        Self {
            key: secret.trim().to_string(),
        }
    }

    pub fn is_valid_public_key(&self) -> bool {
        // Stript the 0x on the key if exist
        let address = self.key.strip_prefix("0x").unwrap_or(&self.key);
        // Verigy the leght and hexadecimal base
        address.len() == 40 && address.chars().all(|c| c.is_digit(16))
    }

    pub fn is_valid_private_key(&self) -> bool {
        // Verify if the key has the correct length
        if self.key.len() != 64 {
            return false;
        }
        // Try to convert the key to bytes
        let bytes = match Vec::from_hex(&self.key) {
            Ok(b) => b,
            Err(_) => return false,
        };
        // Verify if the key is a valid private key
        SecretKey::from_bytes(bytes.as_slice().into()).is_ok()
    }

    pub fn get_key(&self) -> String {
        self.key.clone()
    }
}
