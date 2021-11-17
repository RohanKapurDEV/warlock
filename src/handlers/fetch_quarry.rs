use crate::utils::{network_utils::NetworkConfig, pubkeys::PubkeyConfig, rpc::fetch_account};
use anchor_client::anchor_lang::AccountDeserialize;
use axum::{http::StatusCode, Json};
use quarry_mine::Quarry;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use solana_program::pubkey::Pubkey;
use solana_sdk::{account::Account, program_error::ProgramError};
use tracing::Level;

// Fetch the quarry account specified and deserialize to JSON
pub async fn fetch_quarry_handler(
    Json(payload): Json<FetchAccountRequest>,
) -> Result<Json<FetchQuarryResponse>, (StatusCode, Json<serde_json::Value>)> {
    let network = payload.network_config.variant;
    let pubkey = payload.pubkey_config.pubkey;

    let account = fetch_account(&network, &pubkey);

    match account {
        Ok(value) => {
            tracing::event!(Level::INFO, "Account fetch successful - Step 1/2");
            let try_wrap = QuarryWrapper::wrap(&value);

            match try_wrap {
                Ok(value) => {
                    tracing::event!(Level::INFO, "Quarry wrap successful - Step 2/2");
                    Ok(Json(FetchQuarryResponse {
                        network_config: payload.network_config,
                        quarry: value,
                    }))
                }

                Err(_e) => {
                    tracing::event!(Level::ERROR, "Quarry wrap failed - Step 2/2");
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(Value::String("Failed to fetch blockheight".to_string())),
                    ))
                }
            }
        }

        Err(_e) => {
            tracing::event!(Level::ERROR, "Account fetch failed - Step 1/2");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Value::String("Failed to fetch account".to_string())),
            ))
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct FetchAccountRequest {
    pub network_config: NetworkConfig,
    pub pubkey_config: PubkeyConfig,
}
#[derive(Serialize, Deserialize)]
pub struct FetchQuarryResponse {
    pub network_config: NetworkConfig,
    pub quarry: QuarryWrapper,
}

/// Need to build a quarry wrapper because quarry accounts do not implement Serialize and
/// Deserialize by default which is needed for handler response
#[derive(Serialize, Deserialize)]
pub struct QuarryWrapper {
    /// Rewarder who owns this quarry
    pub rewarder_key: Pubkey,
    /// LP token this quarry is designated to
    pub token_mint_key: Pubkey,
    /// Bump.
    pub bump: u8,

    /// Index of the [Quarry].
    pub index: u16,
    /// Decimals on the token [Mint].
    pub token_mint_decimals: u8,
    /// Timestamp when quarry rewards cease
    pub famine_ts: i64,
    /// Timestamp of last checkpoint
    pub last_update_ts: i64,
    /// Rewards per token stored in the quarry
    pub rewards_per_token_stored: u128,
    /// Amount of rewards distributed to the quarry per year.
    pub annual_rewards_rate: u64,
    /// Rewards shared allocated to this quarry
    pub rewards_share: u64,

    /// Total number of tokens deposited into the quarry.
    pub total_tokens_deposited: u64,
    /// Number of [Miner]s.
    pub num_miners: u64,
}

impl QuarryWrapper {
    /// Wrap a normal Quarry into a WrappedQuarry that implements Serialize and Deserialize
    fn wrap(quarry: &Account) -> Result<Self, ProgramError> {
        let res = deserialize_quarry(quarry);

        match res {
            Ok(q) => Ok(Self {
                rewarder_key: q.rewarder_key,
                rewards_per_token_stored: q.rewards_per_token_stored,
                rewards_share: q.rewards_share,
                annual_rewards_rate: q.annual_rewards_rate,
                token_mint_decimals: q.token_mint_decimals,
                token_mint_key: q.token_mint_key,
                total_tokens_deposited: q.total_tokens_deposited,
                bump: q.bump,
                index: q.index,
                famine_ts: q.famine_ts,
                last_update_ts: q.last_update_ts,
                num_miners: q.num_miners,
            }),
            Err(e) => Err(e),
        }
    }
}

fn deserialize_quarry(account: &Account) -> Result<Quarry, ProgramError> {
    let account_data = account.data.clone();
    let raw_bytes: &mut &[u8] = &mut &account_data[..];

    Quarry::try_deserialize(raw_bytes)
}
