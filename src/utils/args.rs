use std::io;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{Shell, generate};

use crate::crypto::{bitcoin_wallet::BtcArgs, solana_wallet::SolanaArgs};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Generate Bitcoin wallet
    Bitcoin(BtcArgs),

    /// Generate Solana wallet
    Solana(SolanaArgs),
    /// Generate shell completion
    Completion { shell: Shell },
}

impl Args {
    pub fn run(&self) {
        match &self.command {
            Command::Bitcoin(btc_args) => btc_args.run(),
            Command::Solana(solana_args) => solana_args.run(),

            Command::Completion { shell } => {
                let mut args = Args::command();
                let bin_name = std::env::current_exe();
                if let Ok(bin_name) = bin_name
                    && let Some(bin_name) = bin_name.file_name()
                    && let Some(bin_name) = bin_name.to_str()
                {
                    generate(*shell, &mut args, bin_name, &mut io::stdout());
                }
            }
        }
    }
}
