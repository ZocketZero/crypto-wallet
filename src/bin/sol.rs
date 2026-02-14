use clap::Parser;
use crypto_wallet::crypto::solana_wallet::SolanaArgs;

fn main() {
    SolanaArgs::parse().run();
}
