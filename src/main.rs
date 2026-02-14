use clap::Parser;
use crypto_wallet::utils::Args;

fn main() {
    let args = Args::parse();
    args.run();
}
