use crate::utils::{fetch_blockheight, network_utils::NetworkConfig};
use crate::Network;
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::Level;

/// Fetch the current blockheight of the specified network
pub async fn fetch_blockheight_handler(
    Json(payload): Json<NetworkConfig>,
) -> Result<Json<FetchBlockheightResponse>, (StatusCode, Json<serde_json::Value>)> {
    let network = payload.variant;
    let blockheight = fetch_blockheight(&network);

    match blockheight {
        Ok(value) => {
            tracing::event!(Level::INFO, "Blockheight fetch request successful");
            Ok(Json(FetchBlockheightResponse {
                network_config: payload.into(),
                blockheight: value,
            }))
        }
        Err(_e) => {
            tracing::event!(Level::ERROR, "Unable to fetch blockheight on {:?}", network);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Value::String("Failed to fetch blockheight".to_string())),
            ))
        }
    }
}

// Fetch blockheight request is simply a network_utils::NetworkConfig object

/// Example response:
///
/// {
///     "network_config":
///     {
///         "variant": "Mainnet"
///     },
///     "blockheight": 96484360
/// }
#[derive(Debug, Serialize, Deserialize)]
pub struct FetchBlockheightResponse {
    network_config: NetworkConfig,
    blockheight: u64,
}

impl FetchBlockheightResponse {
    pub fn new(blockheight: u64, network: Network) -> Self {
        Self {
            network_config: NetworkConfig { variant: network },
            blockheight: blockheight,
        }
    }
}
