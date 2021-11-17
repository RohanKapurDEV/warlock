use crate::utils::*;
use anchor_client::anchor_lang::AccountDeserialize;
use axum::{http::StatusCode, Json};
use quarry_mine::Miner;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use solana_program::pubkey::Pubkey;
use solana_sdk::{account::Account, program_error::ProgramError};
use tracing::Level;

/// Fetch the miner account specified and deserialize to JSON
pub async fn fetch_miner_handler(
    Json(payload): Json<FetchAccountRequest>,
) -> Result<Json<FetchMinerResponse>, (StatusCode, Json<serde_json::Value>)> {
    let network = payload.network_config.variant;
    let pubkey = payload.pubkey_config.pubkey;

    let account = fetch_account(&network, &pubkey);

    match account {
        Ok(value) => {
            tracing::event!(Level::INFO, "Account fetch successful - Step 1/2");
            let try_wrap = MinerWrapper::wrap(&value);

            match try_wrap {
                Ok(value) => {
                    tracing::event!(Level::INFO, "Miner wrap successful - Step 2/2");
                    Ok(Json(FetchMinerResponse {
                        network_config: payload.network_config,
                        miner: value,
                    }))
                }

                Err(_e) => {
                    tracing::event!(Level::ERROR, "Miner wrap failed - Step 2/2");
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(Value::String("Failed to wrap Miner".to_string())),
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

/// Example response
///
/// {
///     "network_config": {
///         "variant": "Mainnet"
///     },
///     "miner": {...} ~ JSON representation of Miner (see MinerWrapper for format)
/// }
///
/// NOTE: All pubkeys will be represented as an array of 32 unsigned 8-bit integers
#[derive(Serialize, Deserialize)]
pub struct FetchMinerResponse {
    pub network_config: NetworkConfig,
    pub miner: MinerWrapper,
}

/// This type is required because Miner accounts do not natively implement Serialize and
/// Deserialize by default which is needed for axum handler response
#[derive(Serialize, Deserialize)]
pub struct MinerWrapper {
    /// Key of the [Quarry] this [Miner] works on.
    pub quarry_key: Pubkey,
    /// Authority who manages this [Miner].
    /// All withdrawals of tokens must accrue to [TokenAccount]s owned by this account.
    pub authority: Pubkey,

    /// Bump.
    pub bump: u8,

    /// [TokenAccount] to hold the [Miner]'s staked LP tokens.
    pub token_vault_key: Pubkey,

    /// Stores the amount of tokens that the [Miner] may claim.
    /// Whenever the [Miner] claims tokens, this is reset to 0.
    pub rewards_earned: u64,

    /// A checkpoint of the [Quarry]'s reward tokens paid per staked token.
    ///
    /// When the [Miner] is initialized, this number starts at 0.
    /// On the first [quarry_mine::stake_tokens], the [Quarry]#update_rewards_and_miner
    /// method is called, which updates this checkpoint to the current quarry value.
    ///
    /// On a [quarry_mine::claim_rewards], the difference in checkpoints is used to calculate
    /// the amount of tokens owed.
    pub rewards_per_token_paid: u128,

    /// Number of tokens the [Miner] holds.
    pub balance: u64,

    /// Index of the [Miner].
    pub index: u64,
}

impl MinerWrapper {
    pub fn wrap(miner: &Account) -> Result<Self, ProgramError> {
        let res = deserialize_miner(miner);

        match res {
            Ok(m) => Ok(Self {
                quarry_key: m.quarry_key,
                authority: m.authority,
                balance: m.balance,
                bump: m.bump,
                rewards_earned: m.rewards_earned,
                rewards_per_token_paid: m.rewards_per_token_paid,
                token_vault_key: m.token_vault_key,
                index: m.index,
            }),
            Err(e) => Err(e),
        }
    }
}

fn deserialize_miner(account: &Account) -> Result<Miner, ProgramError> {
    let account_data = account.data.clone();
    let raw_bytes: &mut &[u8] = &mut &account_data[..];

    Miner::try_deserialize(raw_bytes)
}
