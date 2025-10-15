//! ANDE Executor Builder
//!
//! Custom executor builder that injects ANDE precompiles into the EVM.
//! Based on reth's precompile-cache example.

use evolve_ev_reth::evm_config::AndeEvmFactory;
use reth_chainspec::ChainSpec;
use reth_ethereum::node::{
    api::{FullNodeTypes, NodeTypes},
    builder::{components::ExecutorBuilder, BuilderContext},
    evm::EthEvmConfig,
};
use reth_ethereum_primitives::EthPrimitives;
use revm::primitives::hardfork::SpecId;

/// Custom executor builder that uses ANDE EVM factory with custom precompiles
#[derive(Debug, Clone, Default)]
pub struct AndeExecutorBuilder;

impl<Node> ExecutorBuilder<Node> for AndeExecutorBuilder
where
    Node: FullNodeTypes<Types: NodeTypes<ChainSpec = ChainSpec, Primitives = EthPrimitives>>,
{
    type EVM = EthEvmConfig<ChainSpec, AndeEvmFactory>;

    async fn build_evm(self, ctx: &BuilderContext<Node>) -> eyre::Result<Self::EVM> {
        // Create ANDE EVM factory with Cancun spec
        // TODO: Get actual spec from chain_spec hardfork schedule
        let ande_factory = AndeEvmFactory::new(SpecId::CANCUN);
        
        // Create EVM config with ANDE factory
        let evm_config = EthEvmConfig::new_with_evm_factory(
            ctx.chain_spec(),
            ande_factory,
        );
        
        Ok(evm_config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_executor_builder_creation() {
        let _builder = AndeExecutorBuilder;
        // Builder created successfully
    }
}
