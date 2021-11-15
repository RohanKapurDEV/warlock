use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Network {
    Mainnet,
    Devnet,
    Localnet,
}

/// Network config object for requests to use
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub variant: Network,
}

impl Network {
    /// Takes in a string and returns the corresponding Network variant
    pub fn fetch_variant(network_str: &str) -> Self {
        match network_str {
            "Mainnet" => Self::Mainnet,
            "Devnet" => Self::Devnet,
            "Localnet" => Self::Localnet,
            &_ => Self::Mainnet, // Simply return mainnet on incorrect option :)
        }
    }

    /// Takes in a network variant and returns the corresponding string
    pub fn fetch_str<'a>(network: Network) -> &'a str {
        match network {
            Self::Mainnet => "Mainnet",
            Self::Devnet => "Devnet",
            Self::Localnet => "Localnet",
        }
    }

    /// Takes in a Network variant and returns an rpc url
    pub fn fetch_url(self) -> String {
        match self {
            Self::Mainnet => "https://solana-api.projectserum.com".to_string(),
            Self::Devnet => "https://api.devnet.solana.com".to_string(),
            Self::Localnet => "http://localhost:8899".to_string(),
        }
    }
}
