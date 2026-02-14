use bitcoin::{Address, Network};
use secp256k1::{PublicKey, SecretKey};

use crate::{
    crypto::bitcoin_wallet::generate::{generate_keypair_from_hash, generate_keypair_from_text},
    utils::{self, PrintMode},
    wallet::{Wallet, WalletTrait},
};

pub struct BtcWallet {
    pub private_key_wif: String,
    pub public_key_str: String,
    pub address: Address,
}

impl WalletTrait for BtcWallet {
    fn to_wallet(&self) -> crate::wallet::Wallet {
        Wallet {
            private_key: self.private_key_wif.clone(),
            public_key: self.address.to_string(),
        }
    }
    fn random() -> Self {
        let hash = utils::random_hash();
        Self::from_hash(&hash, false)
    }
    fn from_file(path: &str) -> Self {
        match utils::hash_from_file(path) {
            Ok(_as) => Self::from_hash(&_as, false),
            Err(_) => panic!("Failed open file"),
        }
    }

    fn from_hash(hash: &[u8; 32]) -> Self {
        Self::from_hash(hash, false)
    }

    fn from_text(text: &str) -> Self {
        BtcWallet::new(text, false)
    }
}

impl BtcWallet {
    pub fn gen_wallet(secret_key: SecretKey, pub_key_secp: PublicKey, compressed: bool) -> Self {
        // Construct bitcoin::PrivateKey with specified compressed state
        let private_key = bitcoin::PrivateKey {
            compressed,
            network: Network::Bitcoin,
            inner: secret_key, // Corrected field name
        };
        let private_key_wif = private_key.to_wif();

        // Construct bitcoin::PublicKey with specified compressed state
        let public_key_bitcoin = bitcoin::PublicKey {
            compressed,
            inner: pub_key_secp,
        };
        let public_key_str = public_key_bitcoin.to_string();

        // Convert to a Bitcoin Address for a sharable format
        let address = Address::p2pkh(&public_key_bitcoin, Network::Bitcoin);

        Self {
            private_key_wif,
            public_key_str,
            address,
        }
    }

    pub fn from_hash(hash: &[u8; 32], compressed: bool) -> Self {
        let (secret_key, pub_key_secp) = generate_keypair_from_hash(hash);
        Self::gen_wallet(secret_key, pub_key_secp, compressed)
    }

    pub fn new(text: &str, compressed: bool) -> Self {
        let (secret_key, pub_key_secp) = generate_keypair_from_text(text);
        Self::gen_wallet(secret_key, pub_key_secp, compressed)
    }

    pub fn print(&self, mode: PrintMode, raw: bool) {
        self.to_wallet().print(mode, raw);
    }
}
