use {
    alloy::primitives::{address, Address},
    core::fmt,
    phf::phf_map,
    std::str::FromStr,
    yttrium::chain_abstraction::{
        api::prepare::Eip155OrSolanaAddress,
        solana::{SolanaPubkey, SOLANA_USDC_ADDRESS},
    },
};

pub const NATIVE_TOKEN_ADDRESS: Address = address!("eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee");

pub struct AssetMetadata {
    pub decimals: u8,
}

/// Asset simulation parameters to override the asset's balance state
pub struct SimulationParams {
    /// Asset contract balance storage slot number per chain
    pub balance_storage_slots: &'static phf::Map<&'static str, u64>,
    /// Balance override for the asset
    pub balance: u128,
}

pub struct AssetEntry {
    pub metadata: AssetMetadata,
    pub simulation: SimulationParams,
    /// Asset contracts per CAIP-2 chain ID
    pub contracts: &'static phf::Map<&'static str, Eip155OrSolanaStatic>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Eip155OrSolanaStatic {
    Eip155(Address),
    Solana(&'static str),
}

impl fmt::Display for Eip155OrSolanaStatic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Eip155OrSolanaStatic::Eip155(address) => address.fmt(f),
            Eip155OrSolanaStatic::Solana(address) => address.fmt(f),
        }
    }
}

impl Eip155OrSolanaStatic {
    pub fn into_eip155_or_solana_address(self) -> Eip155OrSolanaAddress {
        match self {
            Eip155OrSolanaStatic::Eip155(address) => Eip155OrSolanaAddress::Eip155(address),
            Eip155OrSolanaStatic::Solana(address) => {
                Eip155OrSolanaAddress::Solana(SolanaPubkey::from_str(address).unwrap())
            }
        }
    }
}

static USDC_CONTRACTS: phf::Map<&'static str, Eip155OrSolanaStatic> = phf_map! {
    // Optimism
    "eip155:10" => Eip155OrSolanaStatic::Eip155(address!("0b2c639c533813f4aa9d7837caf62653d097ff85")),
    // Base
    "eip155:8453" => Eip155OrSolanaStatic::Eip155(address!("833589fCD6eDb6E08f4c7C32D4f71b54bdA02913")),
    // Arbitrum
    "eip155:42161" => Eip155OrSolanaStatic::Eip155(address!("af88d065e77c8cC2239327C5EDb3A432268e5831")),
    "solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp" => Eip155OrSolanaStatic::Solana(SOLANA_USDC_ADDRESS),
};

static USDT_CONTRACTS: phf::Map<&'static str, Eip155OrSolanaStatic> = phf_map! {
    // Optimism
    "eip155:10" => Eip155OrSolanaStatic::Eip155(address!("94b008aA00579c1307B0EF2c499aD98a8ce58e58")),
    // Arbitrum
    "eip155:42161" => Eip155OrSolanaStatic::Eip155(address!("Fd086bC7CD5C481DCC9C85ebE478A1C0b69FCbb9")),
};

static USDS_CONTRACTS: phf::Map<&'static str, Eip155OrSolanaStatic> = phf_map! {
    // Base
    "eip155:8453" => Eip155OrSolanaStatic::Eip155(address!("820c137fa70c8691f0e44dc420a5e53c168921dc")),
};

static ETH_CONTRACTS: phf::Map<&'static str, Eip155OrSolanaStatic> = phf_map! {
    // Optimism
    "eip155:10" => Eip155OrSolanaStatic::Eip155(NATIVE_TOKEN_ADDRESS),
    // Base
    "eip155:8453" => Eip155OrSolanaStatic::Eip155(NATIVE_TOKEN_ADDRESS),
    // Arbitrum
    "eip155:42161" => Eip155OrSolanaStatic::Eip155(NATIVE_TOKEN_ADDRESS),
};

pub static BRIDGING_ASSETS: phf::Map<&'static str, AssetEntry> = phf_map! {
    "USDC" => AssetEntry {
        metadata: AssetMetadata {
            decimals: 6,
        },
        simulation: SimulationParams {
            // Must be in sync with the `USDC_CONTRACTS` from above
            balance_storage_slots: &phf_map! {
                "eip155:10" => 9u64,
                "eip155:8453" => 9u64,
                "eip155:42161" => 9u64,
            },
            balance: 99000000000,
        },
        contracts: &USDC_CONTRACTS,
    },
    "USDT" => AssetEntry {
        metadata: AssetMetadata {
            decimals: 6,
        },
        simulation: SimulationParams {
            // Must be in sync with the `USDT_CONTRACTS` from above
            balance_storage_slots: &phf_map! {
                "eip155:10" => 0u64,
                "eip155:42161" => 51u64,
            },
            balance: 99000000000,
        },
        contracts: &USDT_CONTRACTS,
    },
    "USDS" => AssetEntry {
        metadata: AssetMetadata {
            decimals: 18,
        },
        simulation: SimulationParams {
            // Must be in sync with the `USDS_CONTRACTS` from above
            balance_storage_slots: &phf_map! {
                "eip155:8453" => 2u64,
            },
            balance: 99000000000000000000000,
        },
        contracts: &USDS_CONTRACTS,
    },
    "ETH" => AssetEntry {
        metadata: AssetMetadata {
            decimals: 18,
        },
        simulation: SimulationParams {
            // Must be in sync with the `ETH_CONTRACTS` from above
            balance_storage_slots: &phf_map! {
                "eip155:10" => 0u64,
                "eip155:8453" => 0u64,
                "eip155:42161" => 0u64,
            },
            balance: 99000000000000000000000,
        },
        contracts: &ETH_CONTRACTS,
    },
};
