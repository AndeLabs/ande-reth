//! Evolve-specific types and integration
//!
//! This crate provides Evolve-specific functionality including:
//! - Custom payload attributes for Evolve
//! - Evolve-specific types and traits
//! - Custom consensus implementation
//! - Custom EVM configuration with ANDE precompiles

/// Evolve-specific types and related definitions.
pub mod types;

/// Configuration for Evolve functionality.
pub mod config;

/// RPC modules for Evolve functionality.
pub mod rpc;

/// Custom consensus implementation for Evolve.
pub mod consensus;

/// Custom EVM configuration with Evolve-specific precompiles.
pub mod evm_config;

/// Parallel EVM execution module.
pub mod parallel;

/// MEV detection and integration module.
pub mod mev;

#[cfg(test)]
mod tests;

// Re-export public types
pub use config::{EvolveConfig, DEFAULT_MAX_TXPOOL_BYTES, DEFAULT_MAX_TXPOOL_GAS};
pub use consensus::{EvolveConsensus, EvolveConsensusBuilder};
pub use evm_config::{ande_token_duality_precompile, ANDE_PRECOMPILE_ADDRESS};
pub use types::{EvolvePayloadAttributes, PayloadAttributesError};
