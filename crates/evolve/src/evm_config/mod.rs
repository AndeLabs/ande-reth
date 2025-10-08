//! Evolve-specific EVM configuration with custom precompiles
//!
//! This module provides the ANDE Token Duality precompile that will be
//! injected into the EVM at runtime during block execution.

pub mod precompile;
pub mod ande_precompile_provider;
pub mod factory;

// Re-export precompile constants and functions for easy access
pub use precompile::{
    ande_token_duality_precompile, ANDE_PRECOMPILE_ADDRESS, ANDE_TOKEN_ADDRESS,
};
pub use ande_precompile_provider::AndePrecompileProvider;
pub use factory::{AndeEvmConfig, create_ande_evm_config};
