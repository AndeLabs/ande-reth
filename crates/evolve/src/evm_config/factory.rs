//! ANDE EVM Configuration for reth v1.7.0
//!
//! This module provides a type alias for EthEvmConfig that enables
//! usage of ANDE-specific configuration while maintaining full compatibility.

use alloy_primitives::Address;
use reth_chainspec::ChainSpec;
use reth_evm_ethereum::EthEvmConfig;
use std::sync::Arc;

/// ANDE EVM Configuration - type alias for EthEvmConfig
///
/// This provides a clean type alias that represents our ANDE-specific EVM configuration
/// while maintaining 100% compatibility with EthEvmConfig.
///
/// In the future, this can be extended to include custom precompile injection.
pub type AndeEvmConfig = EthEvmConfig;

/// Create a new ANDE EVM configuration
pub fn create_ande_evm_config(chain_spec: Arc<ChainSpec>) -> AndeEvmConfig {
    EthEvmConfig::new(chain_spec)
}

/// Get the ANDE precompile address
pub const fn ande_precompile_address() -> Address {
    crate::evm_config::ANDE_PRECOMPILE_ADDRESS
}

#[cfg(test)]
mod tests {
    use super::*;
    use reth_chainspec::{ChainSpecBuilder, MAINNET};
    use reth_node_api::ConfigureEvm; // Importar el trait necesario
    use crate::evm_config::ANDE_PRECOMPILE_ADDRESS;

    #[test]
    fn test_ande_evm_config_creation() {
        let chain_spec = Arc::new(ChainSpecBuilder::default().chain(MAINNET.chain).build());
        let config = create_ande_evm_config(chain_spec);

        // Test that we can create the config
        assert_eq!(config.chain_spec().chain, MAINNET.chain);
    }

    #[test]
    fn test_ande_precompile_address() {
        assert_eq!(ande_precompile_address(), ANDE_PRECOMPILE_ADDRESS);
    }

    #[test]
    fn test_evm_config_compatibility() {
        let chain_spec = Arc::new(ChainSpecBuilder::default().chain(MAINNET.chain).build());
        let config = create_ande_evm_config(chain_spec);

        // Test that we can access all EthEvmConfig functionality
        let _factory = config.block_executor_factory();
        let _assembler = config.block_assembler();

        // These should not panic if everything works
        assert!(!config.chain_spec().chain.named().is_empty());
    }
}