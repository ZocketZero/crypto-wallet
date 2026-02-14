use crate::utils::PrintMode;

pub struct Wallet {
    pub private_key: String,
    pub public_key: String,
}

pub trait WalletTrait {
    fn from_file(path: &str) -> Self;
    fn from_hash(hash: &[u8; 32]) -> Self;
    fn from_text(text: &str) -> Self;
    fn random() -> Self;
    fn to_wallet(&self) -> Wallet;
}

impl Wallet {
    pub fn print(&self, print_mode: PrintMode, raw: bool) {
        if print_mode.is_secret() {
            if !raw {
                print!("Private key: ");
            }
            print!("{}", self.private_key);
            if !raw {
                println!();
            }
        }
        if print_mode.is_public() {
            if !raw {
                print!("Public key: ");
            }
            print!("{}", self.public_key);
            if !raw {
                println!();
            }
        }
    }
}
