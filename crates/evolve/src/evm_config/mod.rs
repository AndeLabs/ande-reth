//! Evolve-specific EVM configuration with custom precompiles
//!
//! This module provides a custom EVM configuration that extends the standard
//! Ethereum EVM with Evolve-specific precompiles, including the ANDE Token Duality
//! precompile for native balance management.

use reth_chainspec::ChainSpec;
use reth_evm::{ConfigureEvm, ConfigureEvmEnv};
use reth_evm_ethereum::EthEvmConfig;
use reth_ethereum_primitives::{Header, TransactionSigned};
use alloy_primitives::{Address, U256};
use reth_revm::{
    primitives::{
        AnalysisKind, BlobExcessGasAndPrice, BlockEnv, CfgEnv, CfgEnvWithHandlerCfg,
        Env, HandlerCfg, PrecompileSpecId, TxEnv,
    },
    Database, Evm, EvmBuilder,
};
use std::sync::Arc;

pub mod precompile;

use precompile::ANDE_PRECOMPILE_ADDRESS;

/// Evolve-specific EVM configuration that extends Ethereum's EVM with custom precompiles
#[derive(Debug, Clone)]
pub struct EvolveEvmConfig {
    /// The underlying Ethereum EVM configuration
    eth_config: EthEvmConfig,
}

impl EvolveEvmConfig {
    /// Creates a new Evolve EVM configuration with the given chain spec
    pub fn new(chain_spec: Arc<ChainSpec>) -> Self {
        Self {
            eth_config: EthEvmConfig::new(chain_spec),
        }
    }

    /// Returns a reference to the underlying chain spec
    pub fn chain_spec(&self) -> &Arc<ChainSpec> {
        self.eth_config.chain_spec()
    }
}

impl ConfigureEvmEnv for EvolveEvmConfig {
    type Header = Header;
    type Transaction = TransactionSigned;

    fn fill_tx_env(&self, tx_env: &mut TxEnv, transaction: &TransactionSigned, sender: Address) {
        self.eth_config.fill_tx_env(tx_env, transaction, sender)
    }

    fn fill_cfg_env(
        &self,
        cfg_env: &mut CfgEnvWithHandlerCfg,
        header: &Self::Header,
        total_difficulty: U256,
    ) {
        self.eth_config
            .fill_cfg_env(cfg_env, header, total_difficulty)
    }

    fn next_cfg_and_block_env(
        &self,
        parent: &Self::Header,
        attributes: reth_evm::NextBlockEnvAttributes,
    ) -> (CfgEnvWithHandlerCfg, BlockEnv) {
        self.eth_config.next_cfg_and_block_env(parent, attributes)
    }
}

impl ConfigureEvm for EvolveEvmConfig {
    type DefaultExternalContext<'a> = ();

    fn evm<DB: Database>(&self, db: DB) -> Evm<'_, (), DB> {
        // For now, delegate to the Ethereum EVM config
        // TODO: Add custom precompile registration
        self.eth_config.evm(db)
    }

    fn evm_with_inspector<DB: Database, I>(
        &self,
        db: DB,
        inspector: I,
    ) -> Evm<'_, I, DB> {
        // For now, delegate to the Ethereum EVM config
        // TODO: Add custom precompile registration with inspector
        self.eth_config.evm_with_inspector(db, inspector)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reth_chainspec::{Chain, ChainSpecBuilder, MAINNET};

    #[test]
    fn test_evolve_evm_config_creation() {
        let chain_spec = Arc::new((*MAINNET).clone());
        let config = EvolveEvmConfig::new(chain_spec.clone());
        assert_eq!(config.chain_spec().chain, chain_spec.chain);
    }

    #[test]
    fn test_ande_precompile_address() {
        // Verify the ANDE precompile is at the expected address
        assert_eq!(
            ANDE_PRECOMPILE_ADDRESS,
            Address::from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfd])
        );
    }
}
