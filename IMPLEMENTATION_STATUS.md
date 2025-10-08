# ANDE Precompile Implementation Status

**Date:** 2025-10-07
**Branch:** `feature/ande-precompile`
**Status:** 🚧 In Progress - Precompile implemented, pending compilation and testing

---

## ✅ Completed

### 1. **Precompile Implementation** (`crates/evolve/src/evm_config/precompile.rs`)

Implemented ANDE Token Duality precompile at address `0x00...fd`:

- ✅ Address constant: `ANDE_PRECOMPILE_ADDRESS = 0x00..fd`
- ✅ Input validation (96 bytes: from, to, value)
- ✅ Zero-address validation
- ✅ Zero-transfer optimization (gas saving)
- ✅ Gas cost calculation (base + per-word)
- ✅ Comprehensive unit tests (7 tests)

**Current Limitations:**
- ⚠️ Caller validation disabled (needs EVM context access)
- ⚠️ Balance transfer stubbed out (needs database access)

**Reason:** The standard precompile function signature (`fn(input: &Bytes, gas_limit: u64) -> PrecompileResult`) doesn't provide access to:
1. `msg.sender` - needed for caller validation
2. EVM database - needed for balance reads/writes

These will be implemented when integrating with the full EVM context.

### 2. **EVM Configuration** (`crates/evolve/src/evm_config/mod.rs`)

Created `EvolveEvmConfig` that extends `EthEvmConfig` with custom precompiles:

- ✅ Extends Ethereum EVM configuration
- ✅ Registers ANDE precompile in handler
- ✅ Implements `ConfigureEvm` trait
- ✅ Implements `ConfigureEvmEnv` trait
- ✅ Unit tests for configuration

### 3. **Module Integration** (`crates/evolve/src/lib.rs`)

- ✅ Exported `evm_config` module
- ✅ Re-exported `EvolveEvmConfig`

---

## 🚧 Next Steps

### 1. **Compile and Fix Errors**

```bash
cd ~/dev/ande-labs/ev-reth
cargo build --release
```

Expected issues to resolve:
- Import statements for `revm` types
- Trait bounds and lifetime issues
- API compatibility with reth v1.7.0

### 2. **Update Builders to Use `EvolveEvmConfig`**

Files to modify:
- `bin/ev-reth/src/builder.rs` - Replace `EthEvmConfig` with `EvolveEvmConfig`
- `crates/node/src/builder.rs` - Update payload builder

Changes needed:
```rust
// BEFORE:
use reth_ethereum::node::EthEvmConfig;
let evm_config = EthEvmConfig::new(chain_spec);

// AFTER:
use evolve_ev_reth::EvolveEvmConfig;
let evm_config = EvolveEvmConfig::new(chain_spec);
```

### 3. **Implement State Access in Precompile**

The precompile needs access to EVM state to:
- Read balances: `db.basic(from)?.balance`
- Update balances: `db.balance_sub(from, value)` and `db.balance_add(to, value)`

Two approaches:
1. **Custom handler approach:** Implement balance logic in `append_handler_register`
2. **Stateful precompile:** Use a different precompile registration that provides DB access

Recommended: Approach #2 using reth's stateful precompile mechanisms.

### 4. **Testing**

Once compilation succeeds:

```bash
# Run all tests
cargo test

# Run precompile-specific tests
cargo test --package evolve-ev-reth --test precompile_tests

# Integration tests
cd ~/dev/ande-labs/andechain
docker compose up -d  # Using ande-reth image
forge test --match-path test/unit/TokenDuality.t.sol --rpc-url http://localhost:8545
```

### 5. **Docker Image Build**

```bash
cd ~/dev/ande-labs/ev-reth
docker build -t andelabs/ande-reth:token-duality-v1 .
docker push ghcr.io/ande-labs/ande-reth:token-duality-v1
```

### 6. **Update andechain**

- Update `infra/docker-compose.yml` to use new image
- Update `ANDETokenDuality.sol` to hardcode precompile address
- Run end-to-end tests

---

## 📝 Implementation Notes

### Precompile Address

```
0x00000000000000000000000000000000000000fd
```

Standard Ethereum precompiles use:
- `0x01` - ECRecover
- `0x02` - SHA256
- `0x03` - RIPEMD160
- `0x04` - Identity
- `0x05` - ModExp
- `0x06-0x09` - BN256 operations
- `0x0a` - Blake2F

We're using `0xfd` (253) for ANDE, well above the standard range.

### Gas Costs

```rust
const ANDE_PRECOMPILE_BASE_GAS: u64 = 3000;
const ANDE_PRECOMPILE_PER_WORD_GAS: u64 = 100;
```

- Base cost: 3000 gas (similar to ModExp)
- Per-word cost: 100 gas (for 32-byte words)
- Total for 96-byte input: 3000 + (3 * 100) = **3300 gas**

For comparison:
- ECRecover: 3000 gas
- SHA256: 60 + 12 per word
- Identity: 15 + 3 per word

### Security Considerations

1. **Caller Validation:** Must implement check that only ANDEToken contract can call
2. **Reentrancy:** Not applicable (precompile can't make external calls)
3. **Integer Overflow:** Using Rust's `U256` prevents overflows
4. **DOS Protection:** Gas metering prevents infinite loops

### Integration with ANDEToken

The ANDEToken contract calls the precompile like:
```solidity
bytes memory input = abi.encode(from, to, value);
(bool success, bytes memory returnData) = _nativeTransferPrecompile.call(input);
```

Expected input format:
```
Offset  | Length | Content
--------|--------|--------------------
0-31    | 32     | from address (left-padded)
32-63   | 32     | to address (left-padded)
64-95   | 32     | value (uint256)
```

Expected output:
```
0x0000000000000000000000000000000000000000000000000000000000000001  // Success
```

---

## 🐛 Known Issues

1. **Caller validation disabled** - Needs EVM context access
2. **Balance transfer stubbed** - Needs database access
3. **Not compiled yet** - May have type/API errors

---

## 📚 References

- **reth documentation:** https://paradigmxyz.github.io/reth/
- **revm precompiles:** https://github.com/bluealloy/revm/tree/main/crates/precompile
- **EIP-2612 (Permit):** https://eips.ethereum.org/EIPS/eip-2612
- **Token Duality PoC:** `~/dev/ande-labs/andechain/docs/token-duality/POC_SUMMARY.md`

---

**Last Updated:** 2025-10-07
**Maintained By:** Ande Labs Development Team
