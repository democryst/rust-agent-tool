use crate::ports::CryptoService;
use sha2::{Sha256, Digest};
use hex;

pub struct Sha2CryptoAdapter;

impl CryptoService for Sha2CryptoAdapter {
    fn hash_sha256(&self, data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        hex::encode(result)
    }

    fn verify_signature(&self, data: &str, signature: &str) -> bool {
        let expected = self.hash_sha256(data);
        expected == signature
    }
}
