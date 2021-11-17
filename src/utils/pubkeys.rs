use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;

pub const QUARRY_MINE_PUBKEY: &str = "QMNeHCGYnLVDn1icRAfQZpjPLBNkfGbSKRB83G5d8KB";

#[derive(Serialize, Deserialize)]
pub struct PubkeyConfig {
    pub pubkey: Pubkey,
}
