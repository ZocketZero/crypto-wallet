use anyhow::anyhow;
use rand::Rng;
use sha2::{Digest, Sha256};
use std::{fs::File, io::Read};

pub fn random_hash() -> [u8; 32] {
    let mut data = [0u8; 32];
    let mut rng = rand::rng();
    rng.fill(&mut data[..]);
    data
}

/// convert sha256 hash from string to unsign 8 bit array
///
/// hash: 2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824
pub fn read_hash(hash: &str) -> anyhow::Result<[u8; 32]> {
    let hash = hex::decode(hash);
    if let Ok(hash) = hash
        && hash.len() == 32
    {
        let hash: [u8; 32] = hash.as_slice().try_into()?;
        return Ok(hash);
    }
    Err(anyhow!("Invalid hash"))
}

pub fn hash_text(text: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    hasher.finalize().into()
}

pub fn hash_from_file(path: &str) -> anyhow::Result<[u8; 32]> {
    if let Ok(mut file) = File::open(path) {
        let mut hasher = Sha256::new();
        let mut file_len = if let Ok(metadata) = file.metadata() {
            metadata.len()
        } else {
            return Err(anyhow!("Failed to read file's metadata"));
        };

        loop {
            let len = if file_len >= 10 {
                file_len -= 10;
                10
            } else {
                file_len
            };

            let mut buf = vec![0u8; len as usize];
            if let Err(err) = file.read_exact(&mut buf) {
                eprintln!("{}", err);
            }
            hasher.update(&buf);
            if len < 10 {
                break;
            }
        }
        let hash: [u8; 32] = hasher.finalize().into();
        Ok(hash)
    } else {
        Err(anyhow!("Cannot open file"))
    }
}
