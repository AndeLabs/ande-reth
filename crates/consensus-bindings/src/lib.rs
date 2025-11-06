//! Ande Consensus Contract Bindings
//!
//! This crate provides Rust bindings for the AndeConsensus smart contract
//! and related contracts for integration with ev-reth.
//!
//! ## Architecture
//!
//! - `AndeConsensus`: Main consensus contract with proposer selection
//! - `AndeNativeStaking`: Staking contract with voting power calculation
//! - `AndeSequencerRegistry`: Sequencer registration and management

#![warn(missing_docs, unreachable_pub)]
#![deny(unused_must_use, rust_2018_idioms)]

use alloy::sol;

// Generate Rust bindings from Solidity contract ABI
sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, PartialEq, Eq)]
    AndeConsensus,
    "../../../andechain/out/AndeConsensus.sol/AndeConsensus.json"
}

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, PartialEq, Eq)]
    AndeNativeStaking,
    "../../../andechain/out/AndeNativeStaking.sol/AndeNativeStaking.json"
}

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, PartialEq, Eq)]
    AndeSequencerRegistry,
    "../../../andechain/out/AndeSequencerRegistry.sol/AndeSequencerRegistry.json"
}

// Re-export main contract types
pub use AndeConsensus::*;
pub use AndeNativeStaking::*;
pub use AndeSequencerRegistry::*;

/// Contract addresses configuration
#[derive(Debug, Clone, Copy)]
pub struct ContractAddresses {
    /// AndeConsensus contract address
    pub consensus: alloy_primitives::Address,
    /// AndeNativeStaking contract address
    pub staking: alloy_primitives::Address,
    /// AndeSequencerRegistry contract address
    pub sequencer_registry: alloy_primitives::Address,
}

impl ContractAddresses {
    /// Create new contract addresses from environment variables
    ///
    /// Expected environment variables:
    /// - `ANDE_CONSENSUS_ADDRESS`
    /// - `ANDE_STAKING_ADDRESS`
    /// - `ANDE_SEQUENCER_REGISTRY_ADDRESS`
    pub fn from_env() -> eyre::Result<Self> {
        let consensus = std::env::var("ANDE_CONSENSUS_ADDRESS")?
            .parse()
            .map_err(|e| eyre::eyre!("Invalid consensus address: {}", e))?;
            
        let staking = std::env::var("ANDE_STAKING_ADDRESS")?
            .parse()
            .map_err(|e| eyre::eyre!("Invalid staking address: {}", e))?;

        let sequencer_registry = std::env::var("ANDE_SEQUENCER_REGISTRY_ADDRESS")?
            .parse()
            .map_err(|e| eyre::eyre!("Invalid sequencer registry address: {}", e))?;
            
        Ok(Self { consensus, staking, sequencer_registry })
    }
    
    /// Create new contract addresses manually
    pub const fn new(
        consensus: alloy_primitives::Address,
        staking: alloy_primitives::Address,
        sequencer_registry: alloy_primitives::Address,
    ) -> Self {
        Self { consensus, staking, sequencer_registry }
    }

    /// Create from hex strings (useful for testing)
    pub fn from_hex(
        consensus: &str,
        staking: &str,
        sequencer_registry: &str,
    ) -> eyre::Result<Self> {
        let consensus = consensus.parse()
            .map_err(|e| eyre::eyre!("Invalid consensus address: {}", e))?;
        let staking = staking.parse()
            .map_err(|e| eyre::eyre!("Invalid staking address: {}", e))?;
        let sequencer_registry = sequencer_registry.parse()
            .map_err(|e| eyre::eyre!("Invalid sequencer registry address: {}", e))?;
        
        Ok(Self::new(consensus, staking, sequencer_registry))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_contract_addresses_new() {
        let consensus = "0x5FbDB2315678afecb367f032d93F642f64180aa3".parse().unwrap();
        let staking = "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512".parse().unwrap();
        let sequencer_registry = "0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0".parse().unwrap();
        
        let addresses = ContractAddresses::new(consensus, staking, sequencer_registry);
        assert_eq!(addresses.consensus, consensus);
        assert_eq!(addresses.staking, staking);
        assert_eq!(addresses.sequencer_registry, sequencer_registry);
    }

    #[test]
    fn test_contract_addresses_from_hex() {
        let addresses = ContractAddresses::from_hex(
            "0x5FbDB2315678afecb367f032d93F642f64180aa3",
            "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512",
            "0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0",
        ).unwrap();
        
        assert_eq!(
            addresses.consensus.to_string(),
            "0x5fbdb2315678afecb367f032d93f642f64180aa3"
        );
    }
}