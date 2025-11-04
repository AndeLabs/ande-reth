use alloy_consensus::transaction::Transaction;
use evolve_ev_reth::EvolvePayloadAttributes;
use reth_errors::RethError;
use reth_evm::{
    execute::{BlockBuilder, BlockBuilderOutcome},
    ConfigureEvm, NextBlockEnvAttributes,
};
use evolve_ev_reth::evm_config::AndeEvmConfig;
use evolve_ev_reth::parallel::{ParallelExecutor, ParallelConfig as EvolveParallelConfig};
use evolve_ev_reth::{AndeConsensusClient, BlockAttester, ConsensusConfig};
use reth_payload_builder_primitives::PayloadBuilderError;
use reth_primitives::{TransactionSigned, Header, SealedBlock, SealedHeader, transaction::SignedTransaction};
use reth_provider::{HeaderProvider, StateProviderFactory};
use reth_revm::{database::StateProviderDatabase, State};
use std::sync::Arc;
use tracing::{debug, info, warn, error};
use crate::config::EvolvePayloadBuilderConfig;

/// Payload builder for Evolve Reth node
#[derive(Debug)]
pub struct EvolvePayloadBuilder<Client> {
    /// The client for state access
    pub client: Arc<Client>,
    /// EVM configuration with ANDE precompiles
    pub evm_config: AndeEvmConfig,
    /// Parallel execution configuration
    pub parallel_config: Option<EvolveParallelConfig>,
    /// AndeChain genesis configuration
    pub config: EvolvePayloadBuilderConfig,
    /// Consensus client for PoS multi-sequencer coordination
    pub consensus_client: Option<Arc<AndeConsensusClient>>,
    /// Block attester for signing and submitting blocks
    pub attester: Option<Arc<BlockAttester>>,
}

impl<Client> EvolvePayloadBuilder<Client>
where
    Client: StateProviderFactory + HeaderProvider<Header = Header> + Send + Sync + 'static,
{
    /// Creates a new instance of `EvolvePayloadBuilder`
    pub fn new(
        client: Arc<Client>,
        evm_config: AndeEvmConfig,
        config: EvolvePayloadBuilderConfig,
    ) -> Self {
        // Log AndeChain genesis configuration if present
        if let Some(andechain) = &config.andechain {
            if let Some(name) = &andechain.name {
                info!(
                    target: "andechain",
                    name = %name,
                    version = ?andechain.version,
                    "AndeChain genesis configuration initialized"
                );
            }
            if let Some(icaro) = &andechain.icaro {
                info!(
                    target: "andechain",
                    icaro = %icaro,
                    "K'intu sacred phrase loaded: {}", icaro
                );
            }
            if !andechain.data.is_empty() {
                info!(
                    target: "andechain",
                    data_count = andechain.data.len(),
                    "AndeChain custom genesis data loaded with {} entries",
                    andechain.data.len()
                );
            }
        }

        Self {
            client,
            evm_config,
            parallel_config: None,
            config,
            consensus_client: None,
            attester: None,
        }
    }

    /// Creates a new instance with parallel configuration
    pub fn new_with_parallel(
        client: Arc<Client>,
        evm_config: AndeEvmConfig,
        parallel_config: Option<EvolveParallelConfig>,
        config: EvolvePayloadBuilderConfig,
    ) -> Self {
        // Log AndeChain genesis configuration if present
        if let Some(andechain) = &config.andechain {
            if let Some(name) = &andechain.name {
                info!(
                    target: "andechain",
                    name = %name,
                    version = ?andechain.version,
                    "AndeChain genesis configuration initialized with parallel execution"
                );
            }
        }

        Self {
            client,
            evm_config,
            parallel_config,
            config,
            consensus_client: None,
            attester: None,
        }
    }

    /// Set consensus client for PoS coordination
    pub fn with_consensus(mut self, consensus_client: Arc<AndeConsensusClient>, attester: Arc<BlockAttester>) -> Self {
        info!(
            target: "andechain::consensus",
            "Consensus client and attester initialized for PoS multi-sequencer"
        );
        self.consensus_client = Some(consensus_client);
        self.attester = Some(attester);
        self
    }

    /// Builds a payload using the provided attributes
    pub async fn build_payload(
        &self,
        attributes: EvolvePayloadAttributes,
    ) -> Result<SealedBlock, PayloadBuilderError> {
        // ===== STEP 1: PoS Multi-Sequencer Verification =====
        // Check if we're the designated block producer for this block
        if let Some(consensus_client) = &self.consensus_client {
            let block_number = attributes.timestamp; // Using timestamp as block identifier
            
            // Get designated producer from consensus contract
            match consensus_client.get_block_producer(block_number).await {
                Ok(designated_producer) => {
                    let our_address = attributes.suggested_fee_recipient;
                    
                    if designated_producer != our_address {
                        warn!(
                            target: "andechain::consensus",
                            block = block_number,
                            designated = ?designated_producer,
                            our_address = ?our_address,
                            "❌ Not our turn to produce block - skipping"
                        );
                        return Err(PayloadBuilderError::Internal(
                            RethError::Other(format!(
                                "Not our turn: designated producer is {:?}, we are {:?}",
                                designated_producer, our_address
                            ).into())
                        ));
                    }
                    
                    info!(
                        target: "andechain::consensus",
                        block = block_number,
                        producer = ?our_address,
                        "✅ Verified as designated block producer"
                    );
                }
                Err(e) => {
                    warn!(
                        target: "andechain::consensus",
                        error = %e,
                        "Failed to query designated producer - continuing anyway"
                    );
                }
            }
        }

        // Create a mutable clone of the EVM config to inject the precompile
        let evm_config = self.evm_config.clone();

        // Validate attributes
        attributes
            .validate()
            .map_err(|e| PayloadBuilderError::Internal(RethError::Other(Box::new(e))))?;

        // Get the latest state provider
        let state_provider = self.client.latest().map_err(PayloadBuilderError::other)?;

        // Create a database from the state provider
        let db = StateProviderDatabase::new(&state_provider);
        let mut state_db = State::builder()
            .with_database(db)
            .with_bundle_update()
            .build();

        // Get parent header using the client's HeaderProvider trait
        let parent_header = self
            .client
            .header(&attributes.parent_hash)
            .map_err(PayloadBuilderError::other)?
            .ok_or_else(|| {
                PayloadBuilderError::Internal(RethError::Other("Parent header not found".into()))
            })?;
        let sealed_parent = SealedHeader::new(parent_header, attributes.parent_hash);

        // Create next block environment attributes
        let gas_limit = attributes.gas_limit.ok_or_else(|| {
            PayloadBuilderError::Internal(RethError::Other(
                "Gas limit is required for evolve payloads".into(),
            ))
        })?;

        let next_block_attrs = NextBlockEnvAttributes {
            timestamp: attributes.timestamp,
            suggested_fee_recipient: attributes.suggested_fee_recipient,
            prev_randao: attributes.prev_randao,
            gas_limit,
            parent_beacon_block_root: Some(alloy_primitives::B256::ZERO), // Set to zero for evolve blocks
            // For post-Shanghai/Cancun chains, an empty withdrawals list is valid
            // and ensures version-specific fields are initialized.
            withdrawals: Some(Default::default()),
        };

        // Decide execution mode: parallel vs sequential BEFORE creating builder
        let should_use_parallel = self.should_use_parallel_execution(&attributes.transactions);

        if should_use_parallel {
            info!(
                transaction_count = attributes.transactions.len(),
                "🚀 AndeChain: Using PARALLEL execution mode"
            );
            return self.build_payload_parallel(
                attributes,
                sealed_parent,
                next_block_attrs,
            ).await;
        } else {
            info!(
                transaction_count = attributes.transactions.len(),
                "📋 AndeChain: Using SEQUENTIAL execution mode"
            );
        }

        // Create block builder using the EVM config (for sequential execution)
        let mut builder = evm_config
            .builder_for_next_block(&mut state_db, &sealed_parent, next_block_attrs)
            .map_err(PayloadBuilderError::other)?;

        // Apply pre-execution changes
        builder
            .apply_pre_execution_changes()
            .map_err(|err| PayloadBuilderError::Internal(err.into()))?;
        // Execute transactions sequentially
        tracing::info!(
            transaction_count = attributes.transactions.len(),
            "Evolve payload builder: executing transactions"
        );
        for (i, tx) in attributes.transactions.iter().enumerate() {
            tracing::debug!(
            index = i,
            hash = ?tx.hash(),
            nonce = tx.nonce(),
            gas_price = ?tx.gas_price(),
            gas_limit = tx.gas_limit(),
            "Processing transaction"
            );

            // Convert to recovered transaction for execution
            let recovered_tx = tx.try_clone_into_recovered().map_err(|_| {
                PayloadBuilderError::Internal(RethError::Other(
                    "Failed to recover transaction".into(),
                ))
            })?;

            // Execute the transaction
            match builder.execute_transaction(recovered_tx) {
                Ok(gas_used) => {
                    tracing::debug!(index = i, gas_used, "Transaction executed successfully");
                    debug!(
                        "[debug] execute_transaction ok: index={}, gas_used={}",
                        i, gas_used
                    );
                }
                Err(err) => {
                    // Log the error but continue with other transactions
                    tracing::warn!(index = i, error = ?err, "Transaction execution failed");
                    debug!(
                        "[debug] execute_transaction err: index={}, err={:?}",
                        i, err
                    );
                }
            }
        }

        // Finish building the block - this calculates the proper state root
        let BlockBuilderOutcome {
            execution_result: _,
            hashed_state: _,
            trie_updates: _,
            block,
        } = builder
            .finish(&state_provider)
            .map_err(PayloadBuilderError::other)?;

        let sealed_block = block.sealed_block().clone();
        tracing::info!(
                    block_number = sealed_block.number,
                    block_hash = ?sealed_block.hash(),
                    transaction_count = sealed_block.transaction_count(),
                    gas_used = sealed_block.gas_used,
                    "Evolve payload builder: built block"
        );

        // ===== STEP 2: PoS Block Attestation =====
        // Sign and submit block to consensus contract
        if let Some(attester) = &self.attester {
            info!(
                target: "andechain::consensus",
                block_number = sealed_block.number,
                block_hash = ?sealed_block.hash(),
                "🔏 Attesting block to consensus contract"
            );
            
            match attester.attest_block(sealed_block.number, sealed_block.hash()).await {
                Ok(tx_hash) => {
                    info!(
                        target: "andechain::consensus",
                        block = sealed_block.number,
                        tx = ?tx_hash,
                        "✅ Block attested successfully"
                    );
                }
                Err(e) => {
                    error!(
                        target: "andechain::consensus",
                        block = sealed_block.number,
                        error = %e,
                        "❌ Failed to attest block - continuing anyway"
                    );
                }
            }
        }

        // Return the sealed block
        Ok(sealed_block)
    }

    /// Decide whether to use parallel execution
    fn should_use_parallel_execution(&self, transactions: &[TransactionSigned]) -> bool {
        // If parallel execution is disabled, use sequential
        let parallel_config = match &self.parallel_config {
            Some(config) => config,
            None => return false,
        };

        // Force sequential if configured
        if parallel_config.force_sequential {
            return false;
        }

        // Need minimum number of transactions for parallel execution
        if transactions.len() < parallel_config.min_transactions_for_parallel {
            return false;
        }

        // TODO: Add more sophisticated heuristics
        // - Check transaction complexity
        // - Analyze potential conflicts
        // - Consider gas usage patterns

        true
    }

    /// Build payload using parallel execution
    async fn build_payload_parallel(
        &self,
        attributes: EvolvePayloadAttributes,
        sealed_parent: SealedHeader,
        next_block_attrs: NextBlockEnvAttributes,
    ) -> Result<SealedBlock, PayloadBuilderError> {
        let parallel_config = self.parallel_config.as_ref()
            .ok_or_else(|| PayloadBuilderError::Internal(RethError::Other(
                "Parallel config not set".into()
            )))?;

        info!(
            "🚀 AndeChain: Starting PARALLEL execution with {} workers",
            parallel_config.concurrency_level.get()
        );

        // Create parallel executor
        let parallel_executor = ParallelExecutor::new(parallel_config.clone());

        // Convert transactions - they're already TransactionSigned
        let signed_transactions = attributes.transactions.clone();

        // Execute transactions in parallel
        let parallel_results = parallel_executor.execute_transactions(
            signed_transactions,
            &self.evm_config,
            &sealed_parent,
            next_block_attrs.clone(),
        ).await
        .map_err(|e| PayloadBuilderError::Internal(RethError::Other(format!("Parallel execution failed: {}", e).into())))?;

        info!(
            "✅ AndeChain: Parallel execution completed: {} transactions processed",
            parallel_results.len()
        );

        // For now, we need to execute sequentially to build the actual block
        // TODO: Implement parallel block building
        warn!("⚠️  AndeChain: Falling back to sequential block building (Phase 1 limitation)");

        // Recreate state_db for sequential block building
        let state_provider = self.client.latest().map_err(PayloadBuilderError::other)?;
        let db = StateProviderDatabase::new(&state_provider);
        let mut state_db = State::builder()
            .with_database(db)
            .with_bundle_update()
            .build();

        // Create block builder and execute transactions sequentially for block construction
        let mut builder = self.evm_config
            .builder_for_next_block(&mut state_db, &sealed_parent, next_block_attrs)
            .map_err(PayloadBuilderError::other)?;

        builder
            .apply_pre_execution_changes()
            .map_err(|err| PayloadBuilderError::Internal(err.into()))?;

        // Execute transactions sequentially to build the block
        let mut _total_gas_used = 0u64;
        for (i, tx) in attributes.transactions.iter().enumerate() {
            let recovered_tx = tx.try_clone_into_recovered().map_err(|_| {
                PayloadBuilderError::Internal(RethError::Other(
                    "Failed to recover transaction".into(),
                ))
            })?;

            match builder.execute_transaction(recovered_tx) {
                Ok(gas_used) => {
                    _total_gas_used += gas_used;
                    debug!("Sequential execution for block building: tx {} gas_used {}", i, gas_used);
                }
                Err(err) => {
                    warn!("Sequential execution failed for tx {}: {:?}", i, err);
                }
            }
        }

        // Finish building the block
        let final_state_provider = self.client.latest().map_err(PayloadBuilderError::other)?;
        let BlockBuilderOutcome {
            execution_result: _,
            hashed_state: _,
            trie_updates: _,
            block,
        } = builder
            .finish(&final_state_provider)
            .map_err(PayloadBuilderError::other)?;

        let sealed_block = block.sealed_block().clone();
        info!(
            "🏁 AndeChain: Block built successfully with parallel pre-processing"
        );

        // ===== PoS Block Attestation (Parallel Path) =====
        // Sign and submit block to consensus contract
        if let Some(attester) = &self.attester {
            info!(
                target: "andechain::consensus",
                block_number = sealed_block.number,
                block_hash = ?sealed_block.hash(),
                "🔏 Attesting block to consensus contract (parallel execution)"
            );
            
            match attester.attest_block(sealed_block.number, sealed_block.hash()).await {
                Ok(tx_hash) => {
                    info!(
                        target: "andechain::consensus",
                        block = sealed_block.number,
                        tx = ?tx_hash,
                        "✅ Block attested successfully"
                    );
                }
                Err(e) => {
                    error!(
                        target: "andechain::consensus",
                        block = sealed_block.number,
                        error = %e,
                        "❌ Failed to attest block - continuing anyway"
                    );
                }
            }
        }

        Ok(sealed_block)
    }
}

/// Creates a new payload builder service
pub fn create_payload_builder_service<Client>(
    client: Arc<Client>,
    evm_config: AndeEvmConfig,
    config: EvolvePayloadBuilderConfig,
) -> Option<EvolvePayloadBuilder<Client>>
where
    Client: StateProviderFactory + HeaderProvider<Header = Header> + Send + Sync + 'static,
{
    Some(EvolvePayloadBuilder::new(client, evm_config, config))
}

/// Creates a new payload builder service with parallel configuration
pub fn create_payload_builder_service_with_parallel<Client>(
    client: Arc<Client>,
    evm_config: AndeEvmConfig,
    parallel_config: Option<EvolveParallelConfig>,
    config: EvolvePayloadBuilderConfig,
) -> Option<EvolvePayloadBuilder<Client>>
where
    Client: StateProviderFactory + HeaderProvider<Header = Header> + Send + Sync + 'static,
{
    Some(EvolvePayloadBuilder::new_with_parallel(client, evm_config, parallel_config, config))
}
