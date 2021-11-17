use anchor_client::anchor_lang::AccountDeserialize;
use quarry_mine::{Miner, Quarry, Rewarder};
use solana_account_decoder::UiAccountEncoding;
use solana_client::client_error::ClientError;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_client::rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType};
use solana_program::pubkey::Pubkey;
use solana_sdk::account::Account;
use solana_sdk::commitment_config::{CommitmentConfig, CommitmentLevel};
use solana_sdk::program_error::ProgramError;

use crate::utils::*;

/// Fetch all quarries for any LP token. Returns a vector of Pubkeys.
// pub fn fetch_quarries_for_lp_token(network: &Network, lp_token_mint: &Pubkey) -> Vec<Pubkey> {
//     let quarry_mine_pubkey = Pubkey::from_str(QUARRY_MINE_PUBKEY).unwrap();

//     let token_mint_filter = Memcmp {
//         offset: 8 + 32,
//         bytes: MemcmpEncodedBytes::Bytes(lp_token_mint.to_bytes().into()),
//         encoding: None,
//     };

//     let conf: RpcProgramAccountsConfig = RpcProgramAccountsConfig {
//         filters: Some(vec![RpcFilterType::Memcmp(token_mint_filter)]),
//         account_config: RpcAccountInfoConfig::default(),
//         with_context: None,
//     };

//     let rpc = RpcClient::new_with_commitment(network.fetch_url(), CommitmentConfig::confirmed());
//     let res = rpc
//         .get_program_accounts_with_config(&quarry_mine_pubkey, conf)
//         .unwrap();

//     let mut pubkey_vec = vec![];

//     for item in res.iter() {
//         pubkey_vec.push(item.0)
//     }

//     pubkey_vec
// }

pub fn fetch_and_deserialize_quarry_account(
    network: &Network,
    quarry_pubkey: &Pubkey,
) -> Result<Quarry, ProgramError> {
    let rpc = RpcClient::new_with_commitment(network.fetch_url(), CommitmentConfig::confirmed());
    let account_data = rpc.get_account(quarry_pubkey).unwrap().data;
    let raw_bytes: &mut &[u8] = &mut &account_data[..];

    let quarry: Quarry = Quarry::try_deserialize(raw_bytes)?;
    Ok(quarry)
}

/// Fetches the current blockheight
pub fn fetch_blockheight(network: &Network) -> Result<u64, ClientError> {
    let rpc = RpcClient::new_with_commitment(network.fetch_url(), CommitmentConfig::confirmed());
    rpc.get_block_height()
}

/// Fetches the requested account from the specified network and pubkey
pub fn fetch_account(network: &Network, account_pubkey: &Pubkey) -> Result<Account, ClientError> {
    let rpc = RpcClient::new_with_commitment(network.fetch_url(), CommitmentConfig::confirmed());
    rpc.get_account(account_pubkey)
}

/// Fetches all program accounts and optionally allows for the passing of Memcmp filters.
///
/// NOTE: This function enforces that all every solana_client::rpc_filter::RpcFilterType used
/// for account filtering is of type solana_client::rpc_filter::Memcmp and also enforces that
/// data encoding is done in Base64 (NOT Base58)
pub fn fetch_program_accounts(
    network: &Network,
    program_id: &Pubkey,
    filters: Option<Vec<Memcmp>>,
    commitment: Option<CommitmentLevel>,
) -> Result<Vec<(Pubkey, Account)>, ClientError> {
    let rpc = RpcClient::new_with_commitment(network.fetch_url(), CommitmentConfig::confirmed());
    let mut filters_vec: Vec<RpcFilterType> = Vec::new();

    let conf = RpcAccountInfoConfig {
        encoding: Some(UiAccountEncoding::Base64), // enforce base64
        data_slice: None,                          // enforce no data slice, subject to change
        commitment: if commitment.is_some() {
            Some(CommitmentConfig {
                commitment: commitment.unwrap(),
            })
        } else {
            None
        },
    };

    if filters.is_some() {
        for item in filters.unwrap() {
            filters_vec.push(RpcFilterType::Memcmp(item))
        }
    };

    let program_accounts_config = RpcProgramAccountsConfig {
        filters: if filters_vec.len() > 0 {
            Some(filters_vec)
        } else {
            None
        },
        with_context: None,
        account_config: conf,
    };

    rpc.get_program_accounts_with_config(program_id, program_accounts_config)
}
