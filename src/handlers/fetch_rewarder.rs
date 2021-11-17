use crate::utils::*;
use anchor_client::anchor_lang::AccountDeserialize;
use axum::{http::StatusCode, Json};
use quarry_mine::Rewarder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use solana_program::pubkey::Pubkey;
use solana_sdk::{account::Account, program_error::ProgramError};
use tracing::Level;

/// Fetch the rewarder account specified and deserialize to JSON
pub async fn fetch_rewarder_handler(
    Json(payload): Json<FetchAccountRequest>,
) -> Result<Json<FetchRewarderResponse>, (StatusCode, Json<serde_json::Value>)> {
    let network = payload.network_config.variant;
    let pubkey = payload.pubkey_config.pubkey;

    let account = fetch_account(&network, &pubkey);

    match account {
        Ok(value) => {
            tracing::event!(Level::INFO, "Account fetch successful - Step 1/2");
            let try_wrap = RewarderWrapper::wrap(&value);

            match try_wrap {
                Ok(value) => {
                    tracing::event!(Level::INFO, "Rewarder wrap successful - Step 2/2");
                    Ok(Json(FetchRewarderResponse {
                        network_config: payload.network_config,
                        rewarder: value,
                    }))
                }

                Err(_e) => {
                    tracing::event!(Level::ERROR, "Rewarder wrap failed - Step 2/2");
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(Value::String("Failed to wrap Rewarder".to_string())),
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
///     "rewarder": {...} ~ JSON representation of Rewarder (see RewarderWrapper for format)
/// }
///
/// NOTE: All pubkeys will be represented as an array of 32 unsigned 8-bit integers
#[derive(Serialize, Deserialize)]
pub struct FetchRewarderResponse {
    pub network_config: NetworkConfig,
    pub rewarder: RewarderWrapper,
}

/// This type is required because rewarder accounts do not natively implement Serialize and
/// Deserialize by default which is needed for axum handler response
#[derive(Serialize, Deserialize)]
pub struct RewarderWrapper {
    /// Random pubkey used for generating the program address.
    pub base: Pubkey,
    /// Bump seed for program address.
    pub bump: u8,

    /// Authority who controls the rewarder
    pub authority: Pubkey,
    /// Pending authority which must accept the authority
    pub pending_authority: Pubkey,

    /// Number of [Quarry]s the [Rewarder] manages.
    /// If more than this many [Quarry]s are desired, one can create
    /// a second rewarder.
    pub num_quarries: u16,
    /// Amount of reward tokens distributed per day
    pub annual_rewards_rate: u64,
    /// Total amount of rewards shares allocated to [Quarry]s
    pub total_rewards_shares: u64,
    /// Mint wrapper.
    pub mint_wrapper: Pubkey,
    /// Mint of the rewards token for this [Rewarder].
    pub rewards_token_mint: Pubkey,

    /// Claim fees are placed in this account.
    pub claim_fee_token_account: Pubkey,
    /// Maximum amount of tokens to send to the Quarry DAO on each claim,
    /// in terms of thousands of BPS.
    /// This is stored on the [Rewarder] to ensure that the fee will
    /// not exceed this in the future.
    pub max_claim_fee_kbps: u64,

    /// Authority allowed to pause a [Rewarder].
    pub pause_authority: Pubkey,
    /// If true, all instructions on the [Rewarder] are paused other than [quarry_mine::unpause].
    pub is_paused: bool,
}

impl RewarderWrapper {
    pub fn wrap(rewarder: &Account) -> Result<Self, ProgramError> {
        let res = deserialize_rewarder(rewarder);

        match res {
            Ok(r) => Ok(Self {
                rewards_token_mint: r.rewards_token_mint,
                base: r.base,
                bump: r.bump,
                annual_rewards_rate: r.annual_rewards_rate,
                total_rewards_shares: r.total_rewards_shares,
                claim_fee_token_account: r.claim_fee_token_account,
                authority: r.authority,
                num_quarries: r.num_quarries,
                max_claim_fee_kbps: r.max_claim_fee_kbps,
                pause_authority: r.pause_authority,
                pending_authority: r.pending_authority,
                mint_wrapper: r.mint_wrapper,
                is_paused: r.is_paused,
            }),
            Err(e) => Err(e),
        }
    }
}

fn deserialize_rewarder(account: &Account) -> Result<Rewarder, ProgramError> {
    let account_data = account.data.clone();
    let raw_bytes: &mut &[u8] = &mut &account_data[..];

    Rewarder::try_deserialize(raw_bytes)
}
