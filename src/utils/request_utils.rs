use crate::utils::{network_utils::NetworkConfig, pubkey_utils::PubkeyConfig};
use serde::{Deserialize, Serialize};

/// Example request
///
/// {
///     "network_config": {
///         "variant": "Mainnet"
///     },
///     "pubkey_config": {
///         "pubkey": [...] ~ an array of 32 unsigned 8-bit integers
///     }
/// }
#[derive(Serialize, Deserialize)]
pub struct FetchAccountRequest {
    pub network_config: NetworkConfig,
    pub pubkey_config: PubkeyConfig,
}
