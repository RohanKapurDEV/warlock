use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;

#[derive(Serialize, Deserialize)]
pub struct PubkeyConfig {
    pub pubkey: Pubkey,
}
