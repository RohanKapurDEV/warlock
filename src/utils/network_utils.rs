#[derive(Clone, Copy, Debug)]
pub enum Network {
    Mainnet,
    Devnet,
    Localnet,
}

impl Network {
    pub fn fetch_url(self) -> String {
        match self {
            Self::Mainnet => "https://solana-api.projectserum.com".to_string(),
            Self::Devnet => "https://api.devnet.solana.com".to_string(),
            Self::Localnet => "http://localhost:8899".to_string(),
        }
    }
}
