# Warlock

Warlock is a tiny web API layer around Quarry Protocol and has endpoints to deserialize Quarry accounts like quarries, miners, and rewarders to JSON. It also has an endpoint to allow you to fetch all miners that belong to a quarry.

## Running Warlock locally

Clone the repo, get into the project root and run the following.

```
cargo build
touch .env && echo "PORT=3000" > .env
cargo run
```

## Request/Response schemas

Before you read the req/res schemas, it's useful to know about some custom types that they use and how they're expected to be passed in from a client.

### Network config

The network config object allows you to specify to an endpoint, a Network that you want it to interact with while performing it's logic. It is implemented in rust like so:

```rust
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Network {
    Mainnet,
    Devnet,
    Localnet,
}

/// Network config object for requests to use
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub variant: Network,
}
```

When building a network config from a client, it is crafted in JSON as follows:

```JSON
{
    "variant": SOME_VARIANT_HERE
}
```

The value for `variant` can be either `"Mainnet"`, `"Devnet"` or `"Localnet"`, with the parantheses since they're strings. If you want to specify custom RPC endpoints for those networks, feel free to edit the `src/utils/network_utils.rs` file locally. By default, it uses the standard Solana public RPC urls.

### Pubkey config

The pubkey config object allows you to specify to an endpoint, a Pubkey that you want it to interact with while performing it's logic. It is implemented in rust like so:

```rust
#[derive(Serialize, Deserialize)]
pub struct PubkeyConfig {
    pub pubkey: Pubkey,
}
```

When building a network config from a client, it is crafted in JSON as follows:

```JSON
{
    "pubkey": [...] // Pubkey array
}
```

The value for `pubkey` should be the byte-array representation of the public key you're passing in which should be an array of 32 unsigned 8-bit integers. If you're using this from a rust client and need to grab the bytes from a pubkey string, you can use the following:

```rust
use solana_program::pubkey::Pubkey;

pub fn pubkey_string_to_bytes(addr: &str) -> [u8; 32] {
    let x = Pubkey::from_str(addr).unwrap();
    x.to_bytes()
}
```

### `/quarry`, `/miner`, `/rewarder`

The request and response schemas for these endpoints are similar enough such that we can group the documentation together.
All 3 endpoints are meant to be called using GET requests and they all use the exact same request format expected to be passed in as JSON in the request body, implemented in Rust as follows:

```rust
#[derive(Serialize, Deserialize)]
pub struct FetchAccountRequest {
    pub network_config: NetworkConfig,
    pub pubkey_config: PubkeyConfig,
}
```

Crafting this request in JSON from a client looks like this:

```JSON
{
    "network_config": {
         "variant": "Mainnet"
     },
     "pubkey_config": {
         "pubkey": [...] // Pubkey array
     }
 }

```

The `"pubkey"` would, once again, be an array of 32 unsigned 8-bit integers, or in other words, the byte-array representation of the pubkey of the quarry, miner, or rewarder you're trying to fetch.

The response schemas for these endpoints are mostly the same with a minor difference, implemented in Rust as follows:

```rust
// getMiner response
#[derive(Serialize, Deserialize)]
pub struct FetchMinerResponse {
    pub network_config: NetworkConfig,
    pub miner: MinerWrapper,
}

// getQuarry response
#[derive(Serialize, Deserialize)]
pub struct FetchQuarryResponse {
    pub network_config: NetworkConfig,
    pub quarry: QuarryWrapper,
}

// getRewarder response
#[derive(Serialize, Deserialize)]
pub struct FetchRewarderResponse {
    pub network_config: NetworkConfig,
    pub rewarder: RewarderWrapper,
}
```

The wrapper accounts (`MinerWrapper`, `QuarryWrapper`, `RewarderWrapper`) have the exact same format as the base accounts from the Quarry Protocol, I just had to reimplement them for reasons that nobody reading this should care about. There are no missing or additional fields in them.

The actual JSON returned from the endpoints would look like this:

```JSON
    // Response for an example getRewarder request. You can imagine what the other two look like based off this :)

    {
    "network_config": {
        "variant": "Mainnet"
    },
    "rewarder": {
        "base": [...], // Pubkey array
        "bump": 255,
        "authority": [...], // Pubkey array
        "pending_authority": [...], // Pubkey array
        "num_quarries": 57,
        "annual_rewards_rate": 511000000000000,
        "total_rewards_shares": 6738,
        "mint_wrapper": [...], // Pubkey array
        "rewards_token_mint": [...], // Pubkey array
        "claim_fee_token_account": [...], // Pubkey array
        "max_claim_fee_kbps": 1000,
        "pause_authority": [...], // Pubkey array
        "is_paused": false
    }
}

```

Each endpoint returns back the network config passed into it and a JSON representation of the onchain account it was called to fetch. The only important thing to note is that for any field where the value is a public key, the value is represented as a byte-array.
