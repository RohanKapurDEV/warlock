use anchor_client::anchor_lang::AccountDeserialize;
use quarry_mine::Quarry;
use solana_client::client_error::ClientError;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_client::rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType};
use solana_program::pubkey::Pubkey;
use solana_sdk::commitment_config::CommitmentConfig;
use std::str::Utf8Error;
use std::{str::FromStr, vec};

mod utils;
use utils::*;

fn main() {
    let x = fetch_and_deserialize_quarry_account(
        &Network::Mainnet,
        &Pubkey::from_str("Hs1X5YtXwZACueUtS9azZyXFDWVxAMLvm3tttubpK7ph").unwrap(),
    );
}

/// Fetch all quarries for any LP token. Returns a vector of Pubkeys.
pub fn fetch_quarries_for_lp(network: &Network, lp_token_mint: &Pubkey) -> Vec<Pubkey> {
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

fn fetch_and_deserialize_quarry_account(
    network: &Network,
    quarry_pubkey: &Pubkey,
) -> Result<Quarry, Utf8Error> {
    let rpc = RpcClient::new_with_commitment(network.fetch_url(), CommitmentConfig::confirmed());
    let account_data = rpc.get_account(quarry_pubkey).unwrap().data;
    let raw_bytes: &mut &[u8] = &mut &account_data[..];

    let quarry: Quarry = Quarry::try_deserialize(raw_bytes).unwrap();
    // Uncomment to test proper deserialization of quarry data
    // println!("{:?}", quarry.token_mint_key.to_string());
    // println!("{:?}", quarry.rewarder_key.to_string());
    Ok(quarry)
}

fn fetch_and_deserialize_miner_account(
    network: &Network,
    miner_pubkey: &Pubkey,
) -> Result<Quarry, Utf8Error> {
    let rpc = RpcClient::new_with_commitment(network.fetch_url(), CommitmentConfig::confirmed());
    let account_data = rpc.get_account(miner_pubkey).unwrap().data;
    let raw_bytes: &mut &[u8] = &mut &account_data[..];

    let quarry: Quarry = Quarry::try_deserialize(raw_bytes).unwrap();
    // Uncomment to test proper deserialization of quarry data
    // println!("{:?}", quarry.token_mint_key.to_string());
    // println!("{:?}", quarry.rewarder_key.to_string());
    Ok(quarry)
}

/// Fetches the current blockheight
fn fetch_blockheight(network: &Network) -> Result<u64, ClientError> {
    let rpc = RpcClient::new_with_commitment(network.fetch_url(), CommitmentConfig::confirmed());
    rpc.get_block_height()
}

pub enum ErrorCode {
    RpcAccountFetchError,
}
