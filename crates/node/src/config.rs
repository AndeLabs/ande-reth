use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// AndeChain Genesis Configuration
/// Contains custom configuration for the AndeChain sovereign rollup
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AndechainGenesisConfig {
    /// K'intu sacred phrase or identifier
    #[serde(default)]
    pub icaro: Option<String>,
    /// Custom hex-encoded data fields
    #[serde(default)]
    pub data: Vec<String>,
    /// Chain name
    #[serde(default)]
    pub name: Option<String>,
    /// Chain version
    #[serde(default)]
    pub version: Option<String>,
    /// Chain type (e.g., "sovereign")
    #[serde(default, rename = "type")]
    pub chain_type: Option<String>,
    /// Network identifier
    #[serde(default)]
    pub network: Option<String>,
    /// Additional custom fields
    #[serde(default, flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Configuration for the Evolve payload builder
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EvolvePayloadBuilderConfig {
    /// AndeChain-specific genesis configuration
    #[serde(default)]
    pub andechain: Option<AndechainGenesisConfig>,
}

impl EvolvePayloadBuilderConfig {
    /// Creates a new instance of `EvolvePayloadBuilderConfig`
    pub const fn new() -> Self {
        Self {
            andechain: None,
        }
    }

    /// Validates the configuration
    pub const fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}

/// Errors that can occur during configuration validation
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    /// Invalid configuration provided
    #[error("Invalid config")]
    InvalidConfig,
}
