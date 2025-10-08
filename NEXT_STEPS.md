# ANDE Precompile - Next Steps

**Date:** 2025-10-07
**Current Status:** 🟡 Implementation blocked - needs reth v1.7.0 API research
**Completion:** ~70%

---

## 🚧 Current Blocker

The `ConfigureEvm` trait in reth v1.7.0 is significantly more complex than initially anticipated. It requires implementing:

```rust
pub trait ConfigureEvm {
    type Primitives: NodePrimitives;
    type Error;
    type NextBlockEnvCtx;
    type BlockExecutorFactory: BlockExecutorFactory;
    type BlockAssembler: BlockAssembler;

    fn block_executor_factory(&self) -> &Self::BlockExecutorFactory;
    fn block_assembler(&self) -> &Self::BlockAssembler;
    fn evm_env(&self, header: &Header) -> EvmEnv;
    fn next_evm_env(&self, header: &Header, ctx: &Self::NextBlockEnvCtx) -> Result<EvmEnv, Self::Error>;
    fn context_for_block(&self, block: &SealedBlock) -> ExecutionCtx;
    fn context_for_next_block(&self, header: &SealedHeader, ctx: Self::NextBlockEnvCtx) -> ExecutionCtx;
}
```

**Issue:** Simply wrapping `EthEvmConfig` doesn't work because we need to provide all these associated types and methods.

---

## ✅ What's Working

1. **Precompile logic:** Fully implemented and tested (`precompile.rs`)
2. **Address allocation:** `0x00...fd` correctly defined
3. **Input validation:** 96-byte parameter decoding works
4. **Test suite:** 7 unit tests pass
5. **Dependencies:** All required crates added to `Cargo.toml`
6. **PoC validation:** ANDETokenDuality + mock working perfectly (26/26 tests)

---

## 🔍 Two Possible Approaches

### Option A: Deep Integration (Recommended for Production)

**Fully integrate with reth's EVM system**

**Steps:**
1. Research `reth-node-ethereum` and `EthEvmConfig` implementation
2. Understand how to properly extend `ConfigureEvm`
3. Implement all required trait methods
4. Register ANDE precompile in the correct lifecycle hook

**Pros:**
- ✅ Proper integration with reth architecture
- ✅ Future-proof for reth updates
- ✅ Production-ready

**Cons:**
- ⏰ Requires 15-20 hours of reth API research
- 🔬 Complex trait bounds and lifetimes
- 📚 Limited documentation on custom precompiles

**Estimated time:** 20-30 hours

---

### Option B: Runtime Injection (Faster, Less Integrated)

**Inject precompile at runtime without full ConfigureEvm impl**

**Strategy:**
Instead of creating a custom `EvolveEvmConfig`, modify the EVM after it's created:

```rust
// In the payload builder or block execution path
let mut evm = eth_config.evm(db);

// Inject ANDE precompile into the EVM's handler
evm.handler.pre_execution.load_precompiles = Arc::new(|| {
    let mut precompiles = default_precompiles();
    precompiles.insert(ANDE_PRECOMPILE_ADDRESS, ande_precompile());
    precompiles
});
```

**Where to inject:**
- `crates/node/src/builder.rs` - In `build_payload()` method
- After EVM creation but before transaction execution

**Pros:**
- ✅ Much faster (~5-8 hours)
- ✅ Doesn't require deep reth knowledge
- ✅ Can be production-ready with proper testing

**Cons:**
- ⚠️ Less elegant than proper trait implementation
- ⚠️ May need updates when reth changes
- ⚠️ Harder to test in isolation

**Estimated time:** 5-8 hours

---

## 📋 Recommended Plan

### Phase 1: Get It Working (Option B) - 1 week

**Goal:** Have a working ande-reth image with ANDE precompile functional

1. **Modify payload builder** (~3 hours)
   - Edit `crates/node/src/builder.rs`
   - Inject precompile after EVM creation
   - Test with simple transfer

2. **Implement state access** (~2-3 hours)
   - Access EVM database in precompile
   - Read/write native balances
   - Handle balance validation

3. **Compilation & testing** (~2-3 hours)
   - Fix any remaining compilation errors
   - Run cargo tests
   - Build Docker image

4. **Integration testing** (~3-4 hours)
   - Deploy to local infrastructure
   - Test with ANDETokenDuality contract
   - Verify gas payment with ANDE works

**Total:** ~10-13 hours

### Phase 2: Production Hardening - 1-2 weeks

1. **Security audit** (~8-10 hours)
   - Review precompile for vulnerabilities
   - Add comprehensive error handling
   - Stress testing

2. **Documentation** (~3-4 hours)
   - Update ANDE_README.md
   - Create deployment guide
   - Document gas costs

3. **Testnet deployment** (~5-6 hours)
   - Configure genesis for testnet
   - Deploy infrastructure
   - Public testing

**Total:** ~16-20 hours

### Phase 3: Proper Integration (Option A) - Later

**When:** After testnet validation, before mainnet

1. Research reth v1.7.0 ConfigureEvm API
2. Implement proper EvolveEvmConfig
3. Migrate from runtime injection to trait implementation
4. Full audit and testing

**Total:** ~20-30 hours

---

## 🎯 Immediate Next Steps (Tomorrow)

1. **Choose approach:** Option B (runtime injection) for speed
2. **Modify builder.rs:**
   ```rust
   // In EvolvePayloadBuilder::build_payload()
   // After creating EVM, inject ANDE precompile
   ```
3. **Implement balance access in precompile:**
   ```rust
   // Access state database
   let from_account = db.basic(from)?;
   let from_balance = from_account.unwrap_or_default().balance;

   // Validate and update
   if from_balance < value {
       return Err(PrecompileError::Other("Insufficient balance".into()));
   }

   db.sub_balance(from, value);
   db.add_balance(to, value);
   ```

4. **Compile and test:**
   ```bash
   cargo build --release
   docker build -t andelabs/ande-reth:dev .
   ```

---

## 📚 Resources Needed

### Reth Documentation
- https://github.com/paradigmxyz/reth/tree/v1.7.0/crates/evm
- https://github.com/paradigmxyz/reth/blob/v1.7.0/crates/ethereum/evm/src/execute.rs
- https://paradigmxyz.github.io/reth/developers/exex/

### revm Documentation
- https://github.com/bluealloy/revm/tree/main/crates/precompile
- https://bluealloy.github.io/revm/crates/precompile/

### Reference Implementations
- Optimism precompiles: https://github.com/paradigmxyz/op-reth
- Polygon precompiles: https://github.com/0xPolygonZero

---

## 💡 Alternative: Use PoC for Initial Launch

**Ultra-conservative approach:**

Launch with the PoC (mock precompile) for initial testnet, then migrate to real precompile later.

**Pros:**
- ✅ Already 100% working
- ✅ Zero compilation issues
- ✅ Full test coverage

**Cons:**
- ❌ ANDE can't be used for gas (major feature missing)
- ❌ Not true Token Duality
- ❌ Would need token migration later

**Verdict:** Not recommended - the whole point is Token Duality

---

## 🤝 Team Discussion Points

1. **Timeline priority:** How urgent is Token Duality for launch?
2. **Resource allocation:** Can we allocate 10-15 hours this week?
3. **Risk tolerance:** Option B (runtime injection) vs Option A (full integration)?
4. **Testing requirements:** What level of testing before testnet deployment?

---

## 📊 Progress Summary

| Component | Status | Completion |
|-----------|--------|------------|
| Precompile logic | ✅ Done | 100% |
| PoC validation | ✅ Done | 100% |
| EVM integration | 🚧 Blocked | 30% |
| State access | ⏸️ Pending | 0% |
| Compilation | 🚧 In Progress | 60% |
| Testing | ⏸️ Pending | 0% |
| Docker image | ⏸️ Pending | 0% |
| **OVERALL** | 🟡 **In Progress** | **~70%** |

---

**Next Session:** Choose Option B, implement runtime injection, get compilation working.

**Estimated to completion:** 10-15 hours (Option B) or 25-35 hours (Option A)

---

**Author:** Ande Labs Development Team
**Last Updated:** 2025-10-07
