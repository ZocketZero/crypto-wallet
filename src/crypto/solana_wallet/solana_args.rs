use crate::{
    crypto::solana_wallet::SolanaWallet,
    utils::{PrintMode, hash_from_file, random_hash, read_hash},
    wallet::WalletTrait,
};
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use std::{io, path::Path};

/// Generate a Solana  wallet from a text seed.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct SolanaArgs {
    /// Text to use as a seed for the wallet generation
    pub seed_text: Option<String>,

    /// use file as seed
    #[arg(short, long, value_name = "File")]
    pub file: Option<String>,

    ///Randomly generate a wallet
    #[arg(long, default_value_t = false)]
    pub random: bool,

    /// generate Solana  from sha256 hash.
    #[arg(long, value_name = "Sha256")]
    pub hash: Option<String>,

    /// Print only specific value.
    #[arg(short, long, default_value = "all")]
    pub print: PrintMode,

    /// Generate Shell completion.
    #[arg(long)]
    pub completion: Option<clap_complete::Shell>,

    /// Print only raw value.
    #[arg(short, long, default_value_t = false)]
    pub raw: bool,
}

impl SolanaArgs {
    pub fn run(&self) {
        if let Some(shell) = self.completion {
            let mut args = SolanaArgs::command();
            let bin_name = std::env::current_exe();
            if let Ok(bin_name) = bin_name
                && let Some(bin_name) = bin_name.file_name()
                && let Some(bin_name) = bin_name.to_str()
            {
                generate(shell, &mut args, bin_name, &mut io::stdout());
            }
        } else if self.random {
            let hash = random_hash();
            SolanaWallet::from_hash(&hash)
                .to_wallet()
                .print(self.print.clone(), self.raw);
        } else if let Some(seed_text) = &self.seed_text {
            let wallet = SolanaWallet::from_text(seed_text);
            wallet.to_wallet().print(self.print.clone(), self.raw);
        } else if let Some(hash) = &self.hash {
            if let Ok(hash) = read_hash(hash) {
                SolanaWallet::from_hash(&hash)
                    .to_wallet()
                    .print(self.print.clone(), self.raw);
            } else {
                eprintln!("Invalid hash");
            }
        } else if let Some(path) = &self.file {
            if Path::new(path).exists()
                && let Ok(hash) = hash_from_file(path)
            {
                SolanaWallet::from_hash(&hash)
                    .to_wallet()
                    .print(self.print.clone(), self.raw);
            } else {
                eprintln!("File not found");
            }
        } else {
            let _ = SolanaArgs::command().print_help();
        }
    }
}
