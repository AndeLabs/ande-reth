//! MEV Distributor Integration
//!
//! Provides interface to interact with the MEVDistributor smart contract
//! for depositing captured MEV and managing epoch distributions.

use alloy_primitives::{Address, U256};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use tracing::{debug, error, info};

/// Epoch data from distributor contract
#[derive(Debug, Clone)]
pub struct EpochData {
    /// Epoch number
    pub epoch: u64,
    /// Total MEV captured in this epoch
    pub total_mev: U256,
    /// Stakers reward amount (80%)
    pub stakers_reward: U256,
    /// Protocol fee amount (15%)
    pub protocol_fee: U256,
    /// Treasury amount (5%)
    pub treasury_amount: U256,
    /// Whether epoch is settled
    pub settled: bool,
    /// Epoch settlement timestamp
    pub timestamp: u64,
}

/// MEV Distributor client for sequencer integration
pub struct MevDistributorClient {
    /// Distributor contract address
    contract_address: Address,
    /// Sequencer address
    sequencer_address: Address,
    /// Accumulated MEV waiting to be deposited
    mev_buffer: Arc<RwLock<U256>>,
    /// Last deposit timestamp
    last_deposit_time: Arc<RwLock<SystemTime>>,
    /// Current epoch number
    current_epoch: Arc<RwLock<u64>>,
    /// Deposit interval
    deposit_interval: Duration,
    /// Maximum MEV buffer before forcing deposit
    max_buffer: U256,
}

impl MevDistributorClient {
    /// Create new distributor client
    pub fn new(
        contract_address: Address,
        sequencer_address: Address,
        deposit_interval: Duration,
        max_buffer: U256,
    ) -> Self {
        Self {
            contract_address,
            sequencer_address,
            mev_buffer: Arc::new(RwLock::new(U256::ZERO)),
            last_deposit_time: Arc::new(RwLock::new(SystemTime::now())),
            current_epoch: Arc::new(RwLock::new(1)),
            deposit_interval,
            max_buffer,
        }
    }
    
    /// Create with default configuration
    pub fn default_config(contract_address: Address, sequencer_address: Address) -> Self {
        Self::new(
            contract_address,
            sequencer_address,
            Duration::from_secs(3600), // 1 hour
            U256::from(1000) * U256::from(10u64.pow(18)), // 1000 ANDE
        )
    }
    
    /// Add MEV to buffer
    pub async fn add_mev(&self, amount: U256) {
        if amount == U256::ZERO {
            return;
        }
        
        let mut buffer = self.mev_buffer.write().await;
        *buffer += amount;
        
        debug!("MEV added to buffer: amount={}, total_buffer={}", amount, *buffer);
        
        // Check if we should deposit immediately
        drop(buffer); // Release lock before checking deposit
        self.check_and_deposit().await;
    }
    
    /// Check if deposit is needed and execute if so
    async fn check_and_deposit(&self) {
        let buffer = *self.mev_buffer.read().await;
        let last_deposit = *self.last_deposit_time.read().await;
        let elapsed = SystemTime::now()
            .duration_since(last_deposit)
            .unwrap_or(Duration::ZERO);
        
        // Deposit if buffer is full or interval elapsed
        let should_deposit = buffer >= self.max_buffer || elapsed >= self.deposit_interval;
        
        if should_deposit && buffer > U256::ZERO {
            if let Err(e) = self.deposit_mev().await {
                error!("Failed to deposit MEV: {}", e);
            }
        }
    }
    
    /// Deposit accumulated MEV to distributor contract
    pub async fn deposit_mev(&self) -> Result<(), String> {
        let amount = {
            let mut buffer = self.mev_buffer.write().await;
            let amount = *buffer;
            
            if amount == U256::ZERO {
                return Ok(());
            }
            
            // Clear buffer
            *buffer = U256::ZERO;
            amount
        };
        
        // Update last deposit time
        {
            let mut last_deposit = self.last_deposit_time.write().await;
            *last_deposit = SystemTime::now();
        }
        
        info!(
            "Depositing MEV to distributor: amount={}, contract={}",
            amount, self.contract_address
        );
        
        // In production, this would call the smart contract
        // self.call_contract_deposit_mev(amount).await?;
        
        Ok(())
    }
    
    /// Force deposit regardless of buffer state
    pub async fn force_deposit(&self) -> Result<U256, String> {
        let amount = *self.mev_buffer.read().await;
        
        if amount > U256::ZERO {
            self.deposit_mev().await?;
            Ok(amount)
        } else {
            Ok(U256::ZERO)
        }
    }
    
    /// Get current buffer amount
    pub async fn get_buffer_amount(&self) -> U256 {
        *self.mev_buffer.read().await
    }
    
    /// Get current epoch data
    pub async fn get_current_epoch(&self) -> EpochData {
        let epoch = *self.current_epoch.read().await;
        
        // In production, this would query the smart contract
        // For now, return mock data
        EpochData {
            epoch,
            total_mev: U256::ZERO,
            stakers_reward: U256::ZERO,
            protocol_fee: U256::ZERO,
            treasury_amount: U256::ZERO,
            settled: false,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Get epoch info for specific epoch number
    pub async fn get_epoch_info(&self, epoch: u64) -> Result<EpochData, String> {
        // In production, this would query the smart contract
        // self.call_contract_get_epoch_data(epoch).await
        
        Ok(EpochData {
            epoch,
            total_mev: U256::ZERO,
            stakers_reward: U256::ZERO,
            protocol_fee: U256::ZERO,
            treasury_amount: U256::ZERO,
            settled: false,
            timestamp: 0,
        })
    }
    
    /// Check if epoch settlement is needed
    pub async fn check_epoch_settlement(&self) -> Result<bool, String> {
        let _current_epoch_data = self.get_current_epoch().await;
        
        // In production, check if epoch duration has passed
        // and call settleEpoch() on the contract if needed
        
        Ok(false)
    }
    
    /// Settle current epoch (admin function)
    pub async fn settle_epoch(&self) -> Result<(), String> {
        let current = *self.current_epoch.read().await;
        
        info!("Settling epoch {}", current);
        
        // In production, this would call the smart contract
        // self.call_contract_settle_epoch().await?;
        
        // Increment epoch
        let mut epoch = self.current_epoch.write().await;
        *epoch += 1;
        
        Ok(())
    }
    
    /// Get distributor statistics
    pub async fn get_distributor_stats(&self) -> DistributorStats {
        let buffer = *self.mev_buffer.read().await;
        let epoch = *self.current_epoch.read().await;
        let last_deposit = *self.last_deposit_time.read().await;
        let time_since_deposit = SystemTime::now()
            .duration_since(last_deposit)
            .unwrap_or(Duration::ZERO);
        
        DistributorStats {
            current_epoch: epoch,
            buffer_amount: buffer,
            time_since_last_deposit: time_since_deposit,
            total_deposited: U256::ZERO, // Would track this in production
            deposits_count: 0,             // Would track this in production
        }
    }
    
    /// Get time until next scheduled deposit
    pub async fn time_until_next_deposit(&self) -> Duration {
        let last_deposit = *self.last_deposit_time.read().await;
        let elapsed = SystemTime::now()
            .duration_since(last_deposit)
            .unwrap_or(Duration::ZERO);
        
        self.deposit_interval.saturating_sub(elapsed)
    }
    
    /// Check if deposit is pending
    pub async fn is_deposit_pending(&self) -> bool {
        let buffer = *self.mev_buffer.read().await;
        buffer >= self.max_buffer
            || self.time_until_next_deposit().await == Duration::ZERO
    }
}

/// Distributor statistics
#[derive(Debug, Clone)]
pub struct DistributorStats {
    /// Current epoch number
    pub current_epoch: u64,
    /// Amount in buffer waiting to be deposited
    pub buffer_amount: U256,
    /// Time since last deposit
    pub time_since_last_deposit: Duration,
    /// Total amount deposited (lifetime)
    pub total_deposited: U256,
    /// Number of deposits made
    pub deposits_count: u64,
}

impl DistributorStats {
    /// Calculate average deposit amount
    pub fn avg_deposit_amount(&self) -> U256 {
        if self.deposits_count > 0 {
            self.total_deposited / U256::from(self.deposits_count)
        } else {
            U256::ZERO
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_distributor_client_creation() {
        let contract = Address::random();
        let sequencer = Address::random();
        let client = MevDistributorClient::default_config(contract, sequencer);
        
        let buffer = client.get_buffer_amount().await;
        assert_eq!(buffer, U256::ZERO);
    }

    #[tokio::test]
    async fn test_add_mev_to_buffer() {
        let contract = Address::random();
        let sequencer = Address::random();
        let client = MevDistributorClient::default_config(contract, sequencer);
        
        let amount = U256::from(1000);
        client.add_mev(amount).await;
        
        let buffer = client.get_buffer_amount().await;
        assert_eq!(buffer, amount);
    }

    #[tokio::test]
    async fn test_deposit_mev() {
        let contract = Address::random();
        let sequencer = Address::random();
        let client = MevDistributorClient::default_config(contract, sequencer);
        
        // Add MEV to buffer
        let amount = U256::from(1000);
        client.add_mev(amount).await;
        
        // Deposit
        let result = client.deposit_mev().await;
        assert!(result.is_ok());
        
        // Buffer should be cleared
        let buffer = client.get_buffer_amount().await;
        assert_eq!(buffer, U256::ZERO);
    }

    #[tokio::test]
    async fn test_force_deposit() {
        let contract = Address::random();
        let sequencer = Address::random();
        let client = MevDistributorClient::default_config(contract, sequencer);
        
        let amount = U256::from(500);
        client.add_mev(amount).await;
        
        let deposited = client.force_deposit().await.unwrap();
        assert_eq!(deposited, amount);
        
        let buffer = client.get_buffer_amount().await;
        assert_eq!(buffer, U256::ZERO);
    }

    #[tokio::test]
    async fn test_epoch_data() {
        let contract = Address::random();
        let sequencer = Address::random();
        let client = MevDistributorClient::default_config(contract, sequencer);
        
        let epoch_data = client.get_current_epoch().await;
        assert_eq!(epoch_data.epoch, 1);
        assert_eq!(epoch_data.total_mev, U256::ZERO);
    }

    #[tokio::test]
    async fn test_distributor_stats() {
        let contract = Address::random();
        let sequencer = Address::random();
        let client = MevDistributorClient::default_config(contract, sequencer);
        
        client.add_mev(U256::from(1000)).await;
        
        let stats = client.get_distributor_stats().await;
        assert_eq!(stats.current_epoch, 1);
        assert_eq!(stats.buffer_amount, U256::from(1000));
    }

    #[tokio::test]
    async fn test_accumulate_multiple_mev() {
        let contract = Address::random();
        let sequencer = Address::random();
        let client = MevDistributorClient::default_config(contract, sequencer);
        
        // Add MEV multiple times
        for i in 1..=5 {
            client.add_mev(U256::from(i * 100)).await;
        }
        
        let buffer = client.get_buffer_amount().await;
        assert_eq!(buffer, U256::from(1500)); // Sum of 100+200+300+400+500
    }
}
