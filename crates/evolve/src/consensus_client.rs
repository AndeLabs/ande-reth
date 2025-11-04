//! Ande Consensus Contract Client
//!
//! Provides integration between ev-reth and the AndeConsensusV2 smart contract
//! for block producer selection, attestation, and validator synchronization.

use alloy::{
    network::EthereumWallet,
    primitives::{Address, Bytes, FixedBytes, B256, U256},
    providers::{Provider, ProviderBuilder, RootProvider},
    signers::local::PrivateKeySigner,
    transports::http::{Client, Http},
};
use ande_consensus_bindings::{AndeConsensusV2, ContractAddresses};
use eyre::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// Ande Consensus Contract Client
/// 
/// Handles all interactions with the AndeConsensusV2 smart contract:
/// - Querying designated block producers
/// - Proposing blocks with signatures
/// - Syncing validator set
/// - Reading validator information
#[derive(Clone)]
pub struct AndeConsensusClient {
    /// Consensus contract instance
    consensus: AndeConsensusV2::AndeConsensusV2Instance<
        Http<Client>,
        Arc<RootProvider<Http<Client>>>,
    >,
    /// Provider for RPC calls
    provider: Arc<RootProvider<Http<Client>>>,
    /// Wallet for signing transactions
    wallet: Option<EthereumWallet>,
    /// Cached active validators
    validators: Arc<RwLock<Vec<Address>>>,
}

impl AndeConsensusClient {
    /// Create a new consensus client
    ///
    /// # Arguments
    /// * `rpc_url` - RPC endpoint URL
    /// * `addresses` - Contract addresses
    /// * `signer` - Optional signer for transactions
    pub async fn new(
        rpc_url: &str,
        addresses: ContractAddresses,
        signer: Option<PrivateKeySigner>,
    ) -> Result<Self> {
        info!("Initializing AndeConsensusClient");
        info!("  RPC URL: {}", rpc_url);
        info!("  Consensus contract: {:?}", addresses.consensus);
        
        // Build provider
        let provider = if let Some(signer) = signer {
            let wallet = EthereumWallet::from(signer.clone());
            ProviderBuilder::new()
                .with_recommended_fillers()
                .wallet(wallet.clone())
                .on_http(rpc_url.parse()?)
        } else {
            ProviderBuilder::new()
                .on_http(rpc_url.parse()?)
        };
        
        let provider = Arc::new(provider);
        
        // Create consensus contract instance
        let consensus = AndeConsensusV2::new(addresses.consensus, provider.clone());
        
        // Initialize empty validator list
        let validators = Arc::new(RwLock::new(Vec::new()));
        
        let client = Self {
            consensus,
            provider,
            wallet: signer.map(EthereumWallet::from),
            validators,
        };
        
        // Initial validator sync
        client.sync_validators().await?;
        
        info!("AndeConsensusClient initialized successfully");
        Ok(client)
    }

    /// Get the designated block producer for a given block number
    ///
    /// This uses the weighted random selection based on stake.
    pub async fn get_block_producer(&self, block_number: u64) -> Result<Address> {
        debug!("Querying block producer for block {}", block_number);
        
        let producer = self
            .consensus
            .getBlockProducer(U256::from(block_number))
            .call()
            .await?
            ._0;
        
        debug!("Block {} producer: {:?}", block_number, producer);
        Ok(producer)
    }

    /// Propose a block to the consensus contract
    ///
    /// This sends a transaction with the block hash and signature.
    /// Requires wallet to be configured.
    pub async fn propose_block(
        &self,
        block_number: u64,
        block_hash: B256,
        signature: Bytes,
    ) -> Result<B256> {
        if self.wallet.is_none() {
            return Err(eyre::eyre!("Wallet not configured, cannot propose blocks"));
        }
        
        info!(
            "Proposing block {} with hash {:?}",
            block_number, block_hash
        );
        
        let tx_hash = self
            .consensus
            .proposeBlock(
                U256::from(block_number),
                block_hash,
                signature,
            )
            .send()
            .await?
            .watch()
            .await?;
        
        info!(
            "Block {} proposed successfully, tx: {:?}",
            block_number, tx_hash
        );
        
        Ok(tx_hash)
    }

    /// Get active validators from the contract
    pub async fn get_active_validators(&self) -> Result<Vec<Address>> {
        debug!("Fetching active validators");
        
        let validators = self.consensus.getActiveValidators().call().await?._0;
        
        debug!("Found {} active validators", validators.len());
        Ok(validators)
    }

    /// Sync validators from contract to local cache
    pub async fn sync_validators(&self) -> Result<()> {
        let validators = self.get_active_validators().await?;
        
        let mut cache = self.validators.write().await;
        *cache = validators.clone();
        
        info!("Synced {} validators to cache", validators.len());
        Ok(())
    }

    /// Get cached validators (fast, no RPC call)
    pub async fn get_cached_validators(&self) -> Vec<Address> {
        self.validators.read().await.clone()
    }

    /// Get validator information
    pub async fn get_validator_info(&self, validator: Address) -> Result<ValidatorInfo> {
        debug!("Fetching info for validator {:?}", validator);
        
        let info = self.consensus.getValidatorInfo(validator).call().await?;
        
        Ok(ValidatorInfo {
            p2p_peer_id: info.p2pPeerId,
            rpc_endpoint: info.rpcEndpoint,
            stake: info.stake,
            power: info.power,
            total_blocks_produced: info.totalBlocksProduced,
            uptime: info.uptime,
            jailed: info.jailed,
            active: info.active,
            is_permanent: info.isPermanent,
        })
    }

    /// Get current epoch number
    pub async fn get_current_epoch(&self) -> Result<u64> {
        let epoch = self.consensus.currentEpoch().call().await?._0;
        Ok(epoch.to())
    }

    /// Get current phase
    pub async fn get_current_phase(&self) -> Result<Phase> {
        let phase = self.consensus.currentPhase().call().await?;
        Ok(phase.into())
    }

    /// Check if address is validator
    pub async fn is_validator(&self, address: Address) -> Result<bool> {
        let is_val = self.consensus.isValidator(address).call().await?._0;
        Ok(is_val)
    }

    /// Get block proposal information
    pub async fn get_block_proposal(&self, block_number: u64) -> Result<Option<BlockProposal>> {
        let proposal = self
            .consensus
            .getBlockProposal(U256::from(block_number))
            .call()
            .await?;
        
        // Check if proposal exists (verified = true)
        if !proposal.verified {
            return Ok(None);
        }
        
        Ok(Some(BlockProposal {
            block_hash: proposal.blockHash,
            producer: proposal.producer,
            signature: proposal.signature,
            timestamp: proposal.timestamp,
            verified: proposal.verified,
        }))
    }
}

/// Validator information
#[derive(Debug, Clone)]
pub struct ValidatorInfo {
    /// libp2p peer ID
    pub p2p_peer_id: FixedBytes<32>,
    /// RPC endpoint URL
    pub rpc_endpoint: String,
    /// Staked amount
    pub stake: U256,
    /// Voting power
    pub power: U256,
    /// Total blocks produced
    pub total_blocks_produced: U256,
    /// Uptime percentage (basis points)
    pub uptime: U256,
    /// Is jailed
    pub jailed: bool,
    /// Is active
    pub active: bool,
    /// Is permanent (genesis validator)
    pub is_permanent: bool,
}

/// Block proposal information
#[derive(Debug, Clone)]
pub struct BlockProposal {
    /// Block hash
    pub block_hash: B256,
    /// Producer address
    pub producer: Address,
    /// Signature
    pub signature: Bytes,
    /// Timestamp
    pub timestamp: U256,
    /// Verified
    pub verified: bool,
}

/// Phase enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    /// Genesis phase (1 validator)
    Genesis,
    /// Dual phase (2 validators)
    Dual,
    /// Multi phase (5-7 validators)
    Multi,
    /// Decentralized phase (20+ validators)
    Decentralized,
}

impl From<u8> for Phase {
    fn from(value: u8) -> Self {
        match value {
            0 => Phase::Genesis,
            1 => Phase::Dual,
            2 => Phase::Multi,
            3 => Phase::Decentralized,
            _ => Phase::Genesis,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_phase_conversion() {
        assert_eq!(Phase::from(0), Phase::Genesis);
        assert_eq!(Phase::from(1), Phase::Dual);
        assert_eq!(Phase::from(2), Phase::Multi);
        assert_eq!(Phase::from(3), Phase::Decentralized);
    }
}
