use clap::Parser;
use crypto_wallet::crypto::bitcoin_wallet::BtcArgs;

fn main() {
    BtcArgs::parse().run();
}
