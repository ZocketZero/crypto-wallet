use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha2::{Digest, Sha256};

pub fn generate_keypair_from_text(text: &str) -> (SecretKey, PublicKey) {
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    let secret_key_bytes: [u8; 32] = hasher.finalize().into();

    generate_keypair_from_hash(&secret_key_bytes)
}

pub fn generate_keypair_from_hash(hash: &[u8; 32]) -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(hash).expect("32 bytes, within curve order");
    let pub_key = PublicKey::from_secret_key(&secp, &secret_key);
    (secret_key, pub_key)
}
