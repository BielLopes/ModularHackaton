use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use ethers::core::k256::SecretKey;
use hex::FromHex;

use crate::errors::Errors;

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

#[cfg(test)]
mod tests {
    use ethers::{core::k256::ecdsa::SigningKey, types::H160, utils::secret_key_to_address};

    fn private_key_to_public_key(private_key_hex: &str) -> String {
        // Converte a chave privada de hexadecimal para bytes
        let private_key_bytes = hex::decode(private_key_hex).expect("Failed to decode private key");

        // Cria uma SigningKey (chave privada) a partir dos bytes
        let signing_key = SigningKey::from_slice(&private_key_bytes).expect("Invalid private key");

        // Obtém o endereço Ethereum associado à chave privada
        let address: H160 = secret_key_to_address(&signing_key);

        // Converte o endereço para uma string no formato "0x" + 40 dígitos hexadecimais
        format!("{:?}", address)
    }

    #[test]
    fn test_private_key_to_public_key() {
        let private_key = "b6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659";
        let address = private_key_to_public_key(private_key);

        // Print the key address
        println!("Address: {}", address);

        // Verifica se a chave pública gerada está correta
        // assert_eq!(
        //     public_key,
        //     "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798"
        // );
    }
}
