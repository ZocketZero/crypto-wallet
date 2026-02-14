use anyhow::anyhow;
use solana_sdk::{
    signature::Keypair,
    signer::{SeedDerivable, Signer},
};

use crate::{utils, wallet::WalletTrait};

pub struct SolanaWallet {
    pub private_key: String,
    pub public_key: String,
}

impl WalletTrait for SolanaWallet {
    fn from_file(path: &str) -> Self {
        let hash = utils::hash_from_file(path);
        let hash = match hash {
            Ok(v) => v,
            Err(_) => panic!("Not found File"),
        };
        Self::from_hash(&hash)
    }

    fn from_hash(hash: &[u8; 32]) -> Self {
        if let Ok(wallet) = SolanaWallet::from_hash_raw(hash) {
            wallet
        } else {
            panic!("Failed");
        }
    }

    fn from_text(text: &str) -> Self {
        Self::from_hash(&utils::hash_text(text))
    }

    fn random() -> Self {
        Self::from_hash(&utils::random_hash())
    }

    fn to_wallet(&self) -> crate::wallet::Wallet {
        crate::wallet::Wallet {
            private_key: self.private_key.clone(),
            public_key: self.public_key.clone(),
        }
    }
}

impl SolanaWallet {
    fn from_hash_raw(hash: &[u8; 32]) -> anyhow::Result<Self> {
        if let Ok(wallet) = Keypair::from_seed(hash) {
            Ok(Self {
                private_key: wallet.to_base58_string(),
                public_key: wallet.pubkey().to_string(),
            })
        } else {
            Err(anyhow!("Failed"))
        }
    }
}
