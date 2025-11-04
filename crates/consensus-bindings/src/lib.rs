//! Ande Consensus Contract Bindings
//!
//! This crate provides Rust bindings for the AndeConsensusV2 smart contract
//! and related contracts (AndeNativeStaking) for integration with ev-reth.

#![warn(missing_docs, unreachable_pub)]
#![deny(unused_must_use, rust_2018_idioms)]

use alloy::sol;

// Generate Rust bindings from Solidity contract ABI
sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, PartialEq, Eq)]
    AndeConsensusV2,
    "../../contracts/out/AndeConsensusV2.sol/AndeConsensusV2.json"
}

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, PartialEq, Eq)]
    AndeNativeStaking,
    "../../contracts/out/AndeNativeStaking.sol/AndeNativeStaking.json"
}

// Re-export main contract types
pub use AndeConsensusV2::*;
pub use AndeNativeStaking::*;

/// Contract addresses configuration
#[derive(Debug, Clone)]
pub struct ContractAddresses {
    /// AndeConsensusV2 contract address
    pub consensus: alloy_primitives::Address,
    /// AndeNativeStaking contract address
    pub staking: alloy_primitives::Address,
}

impl ContractAddresses {
    /// Create new contract addresses from environment variables
    pub fn from_env() -> eyre::Result<Self> {
        let consensus = std::env::var("ANDE_CONSENSUS_ADDRESS")?
            .parse()
            .map_err(|e| eyre::eyre!("Invalid consensus address: {}", e))?;
            
        let staking = std::env::var("ANDE_STAKING_ADDRESS")?
            .parse()
            .map_err(|e| eyre::eyre!("Invalid staking address: {}", e))?;
            
        Ok(Self { consensus, staking })
    }
    
    /// Create new contract addresses manually
    pub fn new(consensus: alloy_primitives::Address, staking: alloy_primitives::Address) -> Self {
        Self { consensus, staking }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_contract_addresses() {
        let consensus = "0x5FbDB2315678afecb367f032d93F642f64180aa3".parse().unwrap();
        let staking = "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512".parse().unwrap();
        
        let addresses = ContractAddresses::new(consensus, staking);
        assert_eq!(addresses.consensus, consensus);
        assert_eq!(addresses.staking, staking);
    }
}
