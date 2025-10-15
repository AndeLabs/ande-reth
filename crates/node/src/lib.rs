//! ev-reth node implementation
//!
//! This crate provides the core node functionality for ev-reth, including:
//! - Payload builder implementation
//! - Node configuration
//! - RPC interfaces

/// Builder module for payload construction and related utilities.
pub mod builder;
/// Configuration types and validation for the Evolve payload builder
pub mod config;
/// Executor builder with ANDE precompiles (experimental)
#[cfg(feature = "experimental")]
pub mod executor_builder;

// Re-export public types
pub use builder::{create_payload_builder_service, EvolvePayloadBuilder};
pub use config::{ConfigError, EvolvePayloadBuilderConfig};

#[cfg(feature = "experimental")]
pub use executor_builder::AndeExecutorBuilder;
