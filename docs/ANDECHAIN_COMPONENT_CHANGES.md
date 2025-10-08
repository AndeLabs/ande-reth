# AndeChain Component Changes - ANDE Integration

**Purpose**: Detailed list of components to add, remove, and modify for ANDE Token Duality integration
**Date**: 2025-10-08
**Impact**: 🔴 **CRITICAL** - Core infrastructure changes

---

## 🔴 COMPONENTS TO ELIMINATE

### 1. **Standard ev-reth Image** ❌ DELETE

**Current Component**:
```yaml
# infra/stacks/single-sequencer/docker-compose.da.local.yml
ev-reth-sequencer:
  image: ghcr.io/evstack/ev-reth:latest
```

**Reason**: Standard ev-reth doesn't include ANDE Token Duality integration
**Replacement**: Custom built `ev-reth:local` image
**Impact**: Critical - Core EVM execution layer

---

## 🟢 COMPONENTS TO ADD

### 1. **Custom Dockerfile** ✅ NEW

**File**: `Dockerfile` (root level)
```dockerfile
FROM rust:1.82-slim as builder

WORKDIR /app
COPY . .

# Build our custom ev-reth with ANDE integration
RUN cargo build --release --bin ev-reth

FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /root

# Copy our custom binary
COPY --from=builder /app/target/release/ev-reth /usr/local/bin/ev-reth

# Expose ports
EXPOSE 8545 8551 9001

# Default command
CMD ["ev-reth"]
```

**Purpose**: Build custom ev-reth with ANDE Token Duality integration
**Dependencies**: Rust 1.82+, our ev-reth source code
**Build Time**: 10-15 minutes (initial), 2-5 minutes (cached)

### 2. **Build Configuration Override** ✅ NEW

**File**: `infra/docker-compose.build.yml`
```yaml
version: '3.8'

services:
  ev-reth-sequencer:
    build:
      context: ../../
      dockerfile: Dockerfile
      target: final
    image: ev-reth:local
    extends:
      file: stacks/single-sequencer/docker-compose.da.local.yml
      service: ev-reth-sequencer
```

**Purpose**: Override default image with custom build
**Usage**: `docker compose -f docker-compose.yml -f docker-compose.build.yml up`

### 3. **ANDE Environment Configuration** ✅ NEW

**File**: `infra/.env.ande`
```env
# ANDE Token Duality Configuration
ANDE_PRECOMPILE_ENABLED=true
ANDE_TOKEN_ADDRESS=0x00000000000000000000000000000000000000FD
ANDE_NETWORK_ID=1234
ANDE_CHAIN_ID=1234

# Build Configuration
EV_RETH_BUILD_TARGET=release
EV_RETH_FEATURES=ande-integration

# Logging
RUST_LOG=info,ev_reth=debug,ande_precompile=trace
```

**Purpose**: Environment variables for ANDE integration
**Integration**: Loaded with docker-compose

### 4. **Build Script** ✅ NEW

**File**: `scripts/build-ev-reth.sh`
```bash
#!/bin/bash
set -e

echo "🔨 Building custom ev-reth with ANDE Token Duality integration..."

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Cargo.toml not found. Run from ev-reth root directory."
    exit 1
fi

# Check Docker is available
if ! command -v docker &> /dev/null; then
    echo "❌ Error: Docker is not installed or not in PATH"
    exit 1
fi

# Build the image
echo "📦 Building Docker image..."
docker build -t ev-reth:local .

# Verify the build
echo "✅ Verifying build..."
if docker run --rm ev-reth:local --version > /dev/null 2>&1; then
    echo "✅ Build successful!"
    echo "🚀 Ready to deploy to AndeChain"
else
    echo "❌ Build failed!"
    exit 1
fi

# Show image size
echo "📊 Image size:"
docker images ev-reth:local
```

**Purpose**: Automated build script for custom ev-reth
**Usage**: `./scripts/build-ev-reth.sh`

### 5. **ANDE Integration Tests** ✅ NEW

**File**: `andechain/test/ande-integration.test.ts`
```typescript
import { expect } from "chai";
import { ethers } from "hardhat";

describe("ANDE Token Duality Integration", function () {
  let andeToken: any;
  let precompileAddress: string;

  beforeEach(async function () {
    // Precompile address
    precompileAddress = "0x00000000000000000000000000000000000000FD";

    // Deploy ANDEToken contract
    const ANDEToken = await ethers.getContractFactory("ANDEToken");
    andeToken = await ANDEToken.deploy(precompileAddress);
    await andeToken.deployed();
  });

  it("Should deploy ANDEToken with correct precompile address", async function () {
    expect(await andeToken.precompileAddress()).to.equal(precompileAddress);
  });

  it("Should execute native balance transfers via precompile", async function () {
    const [owner, recipient] = await ethers.getSigners();

    // Get initial balances
    const initialOwnerBalance = await ethers.provider.getBalance(owner.address);
    const initialRecipientBalance = await ethers.provider.getBalance(recipient.address);

    // Transfer ANDE tokens (should trigger precompile)
    const transferAmount = ethers.utils.parseEther("100");
    await andeToken.transfer(recipient.address, transferAmount);

    // Check if native balances changed (indicating precompile execution)
    const finalOwnerBalance = await ethers.provider.getBalance(owner.address);
    const finalRecipientBalance = await ethers.provider.getBalance(recipient.address);

    // Verify something happened (exact behavior depends on implementation)
    expect(finalOwnerBalance).to.not.equal(initialOwnerBalance);
  });
});
```

**Purpose**: Integration tests for ANDE functionality in AndeChain
**Framework**: Hardhat/Chai
**Coverage**: Precompile execution, native balance transfers

---

## 🟡 COMPONENTS TO MODIFY

### 1. **Docker Compose Configuration** 🔄 UPDATE

**File**: `infra/stacks/single-sequencer/docker-compose.da.local.yml`

**Changes**:

**Line 26** (Image reference):
```yaml
# OLD
image: ghcr.io/evstack/ev-reth:latest

# NEW
image: ev-reth:local
```

**Lines 44-69** (Command configuration):
```yaml
# OLD
command:
  - |
      ev-reth node \
      --ev-reth.enable \
      --engine.persistence-threshold 0 \
      # ... rest of config

# NEW
command:
  - |
      ev-reth node \
      --ev-reth.enable \
      --ev-reth.config=ande \
      --ande.precompile-address=0x00000000000000000000000000000000000000FD \
      --engine.persistence-threshold 0 \
      # ... rest of config
```

**Add environment variables** (after line 30):
```yaml
env_file:
  - .env
  - ../.env.ande
```

### 2. **Main Docker Compose** 🔄 UPDATE

**File**: `infra/docker-compose.yml`

**Add build context** (before line 16):
```yaml
services:
  ev-reth-sequencer:
    build:
      context: ../../
      dockerfile: Dockerfile
    extends:
      file: stacks/single-sequencer/docker-compose.da.local.yml
      service: ev-reth-sequencer

# Keep existing includes
include:
  - ./stacks/da-local/docker-compose.yml
  - ./stacks/single-sequencer/docker-compose.da.local.yml
  - ./stacks/eth-explorer/docker-compose.yml
  - ./stacks/eth-faucet/docker-compose.yml
```

### 3. **Development Makefile** 🔄 UPDATE

**File**: `andechain/Makefile`

**Add new targets**:

```makefile
# Build custom ev-reth
build-ev-reth:
	@echo "🔨 Building custom ev-reth with ANDE integration..."
	@cd ../ev-reth && ./scripts/build-ev-reth.sh

# Start with custom ev-reth
start-ande:
	@echo "🚀 Starting AndeChain with ANDE Token Duality..."
	@$(MAKE) build-ev-reth
	@cd infra && docker compose up -d --force-recreate

# Reset and start with ANDE
reset-ande:
	@echo "🔄 Reset AndeChain with ANDE integration..."
	@cd infra && docker compose down -v
	@$(MAKE) start-ande

# Test ANDE integration
test-ande:
	@echo "🧪 Testing ANDE Token Duality integration..."
	@cd test && npx hardhat test ./ande-integration.test.ts --network localhost
```

**Update existing targets**:

```makefile
# Update start target
start:
	@if [ -f "../ev-reth/Dockerfile" ]; then \
		echo "🚀 Starting AndeChain with ANDE integration..."; \
		$(MAKE) start-ande; \
	else \
		echo "🚀 Starting standard AndeChain..."; \
		./scripts/start-all.sh; \
	fi
```

### 4. **Development Scripts** 🔄 UPDATE

**File**: `andechain/scripts/start-all.sh`

**Add ANDE check**:
```bash
#!/bin/bash

echo "🚀 Starting AndeChain..."

# Check for ANDE integration
if [ -f "../ev-reth/Dockerfile" ]; then
    echo "✅ ANDE integration detected"
    echo "🔨 Building custom ev-reth..."
    cd ../ev-reth
    ./scripts/build-ev-reth.sh
    cd ../andechain/infra

    echo "🚀 Starting with custom ev-reth..."
    docker compose -f docker-compose.yml -f docker-compose.build.yml up -d
else
    echo "ℹ️  Standard ev-reth detected"
    echo "🚀 Starting with standard ev-reth..."
    docker compose up -d
fi

# Wait for services...
echo "⏳ Waiting for services to start..."
sleep 30

# Verify blockchain is running
echo "🔍 Verifying blockchain..."
curl -X POST -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' \
  http://localhost:8545

echo "✅ AndeChain is ready!"
```

### 5. **Documentation** 🔄 UPDATE

**File**: `andechain/CLAUDE.md`

**Add ANDE section**:
```markdown
## ANDE Token Duality Integration

### Overview
AndeChain includes ANDE Token Duality integration through a custom ev-reth build that enables native balance transfers via precompiles.

### Precompile Address
- **Address**: `0x00000000000000000000000000000000000000FD`
- **Purpose**: Native balance transfers for ANDE tokens
- **Compatibility**: Follows Celo's production pattern

### Development Commands

```bash
# Build custom ev-reth with ANDE integration
make build-ev-reth

# Start AndeChain with ANDE integration
make start-ande

# Reset and start fresh with ANDE
make reset-ande

# Test ANDE integration
make test-ande
```

### Architecture with ANDE
```
User/DApp → RPC (localhost:8545)
    ↓
ev-reth-sequencer (CUSTOM ev-reth with ANDE) ✅
    ↓ (Engine API)
single-sequencer (ev-node)
    ↓
local-da (mock Celestia)
```

### Testing ANDE Functionality
1. Deploy ANDEToken contract
2. Call transfer() method
3. Verify precompile execution at 0x00..fd
4. Check native balance changes
```

---

## 🚀 NEW WORKFLOW SUMMARY

### Before Changes
```bash
cd andechain
make start                    # Uses standard ev-reth
make deploy-ecosystem         # Deploys contracts
```

### After Changes
```bash
# One-time setup
cd ev-reth
./scripts/build-ev-reth.sh    # Build custom image

# Regular development
cd andechain
make start-ande              # Uses custom ev-reth with ANDE
make deploy-ecosystem         # Deploys contracts
make test-ande               # Tests ANDE integration
```

### Development Iteration
```bash
# Make changes to ev-reth
cd ev-reth
# Modify ANDE integration code
./scripts/build-ev-reth.sh    # Rebuild

# Restart AndeChain
cd andechain
make reset-ande              # Complete reset with new build
```

---

## 📊 IMPACT ASSESSMENT

### 🔴 Critical Changes
- **EVM Execution Layer**: Complete replacement of standard ev-reth
- **Build Process**: Requires custom Docker build
- **Development Workflow**: New build step required

### 🟡 Moderate Changes
- **Docker Configuration**: Image reference and build context
- **Makefile**: New targets for ANDE integration
- **Scripts**: Updated to detect ANDE integration

### 🟢 Minimal Changes
- **Smart Contracts**: No changes required
- **Genesis Configuration**: No changes required
- **Network Topology**: No changes required

---

## ✅ VERIFICATION CHECKLIST

After implementing changes:

- [ ] Custom ev-reth builds successfully
- [ ] Docker image size < 1GB
- [ ] AndeChain starts without errors
- [ ] ANDE precompile accessible at 0x00..fd
- [ ] Smart contracts deploy correctly
- [ ] Existing functionality preserved
- [ ] New workflow documented
- [ ] Team trained on new process

---

## 🔄 ROLLBACK PLAN

If issues arise:

1. **Quick Rollback**:
   ```bash
   cd andechain/infra
   # Restore original docker-compose.da.local.yml
   git checkout -- stacks/single-sequencer/docker-compose.da.local.yml
   make reset
   make start
   ```

2. **Complete Rollback**:
   ```bash
   cd andechain
   git clean -fd
   git reset --hard HEAD
   make reset
   make start
   ```

3. **Documentation Rollback**:
   ```bash
   git checkout HEAD~1 -- CLAUDE.md Makefile scripts/
   ```

---

**Status**: 📋 **PLANNING COMPLETE** - Ready for implementation
**Risk Level**: 🔴 **HIGH** - Core infrastructure changes
**Testing Required**: Comprehensive integration testing
**Rollback Plan**: Complete rollback procedures documented

---

*This component change plan provides a detailed roadmap for transforming AndeChain from a standard rollup into an ANDE Token Duality-enabled blockchain.*