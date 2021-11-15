use crate::utils::{fetch_blockheight, network_utils::NetworkConfig};
use crate::Network;
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::Level;

pub async fn fetch_blockheight_handler(
    Json(payload): Json<NetworkConfig>,
) -> Result<Json<FetchBlockheightResponse>, (StatusCode, Json<serde_json::Value>)> {
    let network_config: NetworkConfig = payload.into();
    let network = network_config.variant;
    let blockheight = fetch_blockheight(&network);

    match blockheight {
        Ok(value) => Ok(Json(FetchBlockheightResponse::new(value, network))),
        Err(_client_err) => {
            tracing::event!(Level::ERROR, "Unable to fetch blockheight on {:?}", network);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Value::String("Failed to fetch blockheight".to_string())),
            ))
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FetchBlockheightResponse {
    network: Network,
    blockheight: u64,
}

impl FetchBlockheightResponse {
    pub fn new(blockheight: u64, network: Network) -> Self {
        Self {
            network: network,
            blockheight: blockheight,
        }
    }
}
