use anchor_client::anchor_lang::AccountDeserialize;
use quarry_mine::{Miner, Quarry, Rewarder};
use solana_client::client_error::ClientError;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_client::rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType};
use solana_program::pubkey::Pubkey;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::program_error::ProgramError;
use std::{str::FromStr, vec};

use crate::utils::*;

/// Fetch all quarries for any LP token. Returns a vector of Pubkeys.
pub fn fetch_quarries_for_lp_token(network: &Network, lp_token_mint: &Pubkey) -> Vec<Pubkey> {
    let quarry_mine_pubkey = Pubkey::from_str(QUARRY_MINE_PUBKEY).unwrap();

    let token_mint_filter = Memcmp {
        offset: 8 + 32,
        bytes: MemcmpEncodedBytes::Bytes(lp_token_mint.to_bytes().into()),
        encoding: None,
    };

    let conf: RpcProgramAccountsConfig = RpcProgramAccountsConfig {
        filters: Some(vec![RpcFilterType::Memcmp(token_mint_filter)]),
        account_config: RpcAccountInfoConfig::default(),
        with_context: None,
    };

    let rpc = RpcClient::new_with_commitment(network.fetch_url(), CommitmentConfig::confirmed());
    let res = rpc
        .get_program_accounts_with_config(&quarry_mine_pubkey, conf)
        .unwrap();

    let mut pubkey_vec = vec![];

    for item in res.iter() {
        pubkey_vec.push(item.0)
    }

    pubkey_vec
}

pub fn fetch_miners_for_quarry(network: &Network, quarry_pubkey: &Pubkey) {}

pub fn fetch_and_deserialize_quarry_account(
    network: &Network,
    quarry_pubkey: &Pubkey,
) -> Result<Quarry, ProgramError> {
    let rpc = RpcClient::new_with_commitment(network.fetch_url(), CommitmentConfig::confirmed());
    // Rpc call can panic
    let account_data = rpc.get_account(quarry_pubkey).unwrap().data;
    let raw_bytes: &mut &[u8] = &mut &account_data[..];

    let quarry: Quarry = Quarry::try_deserialize(raw_bytes)?;
    // Uncomment to test proper deserialization of quarry data
    // println!("{:?}", quarry.token_mint_key.to_string());
    // println!("{:?}", quarry.rewarder_key.to_string());
    Ok(quarry)
}

pub fn fetch_and_deserialize_miner_account(
    network: &Network,
    miner_pubkey: &Pubkey,
) -> Result<Miner, ProgramError> {
    let rpc = RpcClient::new_with_commitment(network.fetch_url(), CommitmentConfig::confirmed());
    let account_data = rpc.get_account(miner_pubkey).unwrap().data;
    let raw_bytes: &mut &[u8] = &mut &account_data[..];

    let miner: Miner = Miner::try_deserialize(raw_bytes)?;
    // Uncomment to test proper deserialization of miner data
    // println!("{:?}", miner.rewards_earned.to_string());
    // println!("{:?}", miner.balance.to_string());
    Ok(miner)
}

pub fn fetch_and_deserialize_rewarder_account(
    network: &Network,
    rewarder_pubkey: &Pubkey,
) -> Result<Rewarder, ProgramError> {
    let rpc = RpcClient::new_with_commitment(network.fetch_url(), CommitmentConfig::confirmed());
    let account_data = rpc.get_account(rewarder_pubkey).unwrap().data;
    let raw_bytes: &mut &[u8] = &mut &account_data[..];

    let rewarder: Rewarder = Rewarder::try_deserialize(raw_bytes)?;
    // Uncomment to test proper deserialization of miner data
    // println!("{:?}", miner.rewards_earned.to_string());
    // println!("{:?}", miner.balance.to_string());
    Ok(rewarder)
}

/// Fetches the current blockheight
pub fn fetch_blockheight(network: &Network) -> Result<u64, ClientError> {
    let rpc = RpcClient::new_with_commitment(network.fetch_url(), CommitmentConfig::confirmed());
    rpc.get_block_height()
}
