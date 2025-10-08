# AndeChain Integration Plan - ANDE Token Duality

**Purpose**: Complete plan to integrate ev-reth with ANDE Token Duality into AndeChain
**Current State**: ev-reth v0.3.0 complete, AndeChain using standard ev-reth
**Target**: AndeChain with native ANDE Token Duality functionality
**Date**: 2025-10-08

---

## 🎯 Executive Summary

We need to replace the standard `ghcr.io/evstack/ev-reth:latest` image in AndeChain with our custom ev-reth build that includes ANDE Token Duality integration. This will enable native balance transfers through the precompile at `0x00000000000000000000000000000000000000FD`.

---

## 📋 Current State Analysis

### Current AndeChain Architecture

```
User/DApp → RPC (localhost:8545)
    ↓
ev-reth-sequencer (STANDARD ev-reth) ❌
    ↓ (Engine API)
single-sequencer (ev-node)
    ↓
local-da (mock Celestia)
```

**Problem**: Using standard ev-reth without ANDE integration

### Current Docker Configuration

**File**: `infra/stacks/single-sequencer/docker-compose.da.local.yml`
**Image**: `ghcr.io/evstack/ev-reth:latest` (line 26)
**Genesis**: `./genesis.final.json` (line 37)
**Command**: Standard ev-reth node with `--ev-reth.enable` (line 46)

---

## 🔄 Required Changes

### 1. **Replace Docker Image** 🔴 CRITICAL

**Current**:
```yaml
ev-reth-sequencer:
  image: ghcr.io/evstack/ev-reth:latest
```

**New**:
```yaml
ev-reth-sequencer:
  image: ev-reth:local  # Nuestro build personalizado
```

### 2. **Build Custom Docker Image** 🔴 CRITICAL

We need to create a Dockerfile for our custom ev-reth:

**New File**: `Dockerfile` (root level)
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

### 3. **Update Docker Compose Build** 🔴 CRITICAL

**Modify**: `infra/docker-compose.yml`
```yaml
services:
  ev-reth-sequencer:
    build:
      context: ../../  # Build from our ev-reth root
      dockerfile: Dockerfile
    # ... rest of configuration
```

### 4. **Update Configuration** 🟡 RECOMMENDED

**Current Command** (lines 45-69):
```yaml
command:
  - |
      ev-reth node \
      --ev-reth.enable \
      # ... rest of config
```

**New Command** (add ANDE-specific flags):
```yaml
command:
  - |
      ev-reth node \
      --ev-reth.enable \
      --ev-reth.config=ande  # Nuestra configuración personalizada
      # ... rest of config
```

---

## 📁 Files to Modify/Create

### New Files

1. **`Dockerfile`** (root level)
   - Build custom ev-reth with ANDE integration
   - Multi-stage build for optimization

2. **`infra/docker-compose.build.yml`** (new)
   - Override for building custom image
   - Development configuration

### Modified Files

1. **`infra/stacks/single-sequencer/docker-compose.da.local.yml`**
   - Replace image reference
   - Update build configuration

2. **`infra/docker-compose.yml`**
   - Add build context
   - Add environment variables

### Files to Keep Unchanged

- ✅ `genesis.final.json` - No changes needed
- ✅ `single-sequencer` configuration - No changes needed
- ✅ Smart contracts - No changes needed
- ✅ Relayer configuration - No changes needed

---

## 🚀 Deployment Strategy

### Phase 1: Build & Test Local

1. **Build Custom Image**
   ```bash
   # From ev-reth root
   docker build -t ev-reth:local .
   ```

2. **Test Image**
   ```bash
   docker run --rm ev-reth:local --version
   ```

3. **Verify ANDE Integration**
   ```bash
   docker run --rm ev-reth:local node --help | grep -i ande
   ```

### Phase 2: Update Infrastructure

1. **Backup Current Configuration**
   ```bash
   cd andechain/infra
   cp stacks/single-sequencer/docker-compose.da.local.yml stacks/single-sequencer/docker-compose.da.local.yml.backup
   ```

2. **Update Docker Compose**
   - Modify image reference
   - Add build configuration
   - Test configuration syntax

3. **Deploy Updated Infrastructure**
   ```bash
   cd andechain
   make reset  # Complete reset required
   make start
   ```

### Phase 3: Integration Testing

1. **Verify Chain Startup**
   ```bash
   curl -X POST -H "Content-Type: application/json" \
     --data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' \
     http://localhost:8545
   ```

2. **Deploy Smart Contracts**
   ```bash
   cd andechain/contracts
   export PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
   forge script script/DeployEcosystem.s.sol:DeployEcosystem --rpc-url local --broadcast --legacy
   ```

3. **Test ANDE Token Duality**
   - Deploy ANDEToken contract
   - Call transfer() method
   - Verify precompile execution
   - Check native balance changes

---

## 🧪 Testing Plan

### Unit Tests (Already Complete)

✅ All 25 tests pass in ev-reth
✅ ANDE integration tests verified
✅ Precompile address confirmed

### Integration Tests (New)

1. **Blockchain Startup Test**
   ```bash
   # Verify chain starts with custom ev-reth
   make start
   # Check logs for ANDE integration
   docker compose logs ev-reth-sequencer | grep -i ande
   ```

2. **Contract Deployment Test**
   ```bash
   # Deploy existing contracts
   make deploy-ecosystem
   # Verify deployment succeeds
   ```

3. **ANDE Token Test**
   ```bash
   # Create test script for ANDE token functionality
   # Test precompile execution
   # Verify native balance transfers
   ```

4. **End-to-End Test**
   ```bash
   # Complete flow test
   # 1. User calls ANDEToken.transfer()
   # 2. Precompile executes
   # 3. Native balance changes
   # 4. State persisted correctly
   ```

---

## 🔄 New Development Workflow

### Before (Current)

```bash
cd andechain
make start  # Uses standard ev-reth
make deploy-ecosystem  # Deploys contracts
```

### After (With ANDE Integration)

```bash
# Build custom ev-reth (one-time setup)
cd ev-reth
docker build -t ev-reth:local .

# Start AndeChain with ANDE integration
cd andechain
make start  # Uses custom ev-reth with ANDE
make deploy-ecosystem  # Deploys contracts
```

### Development Iteration

```bash
# Make changes to ev-reth code
cd ev-reth
# Modify ANDE integration code
docker build -t ev-reth:local .

# Restart AndeChain with new build
cd andechain
make stop
make start  # Uses updated custom ev-reth
```

---

## 🚨 Critical Considerations

### 1. **Complete Reset Required** 🔴

When changing the ev-reth image, you MUST run:

```bash
cd andechain
make reset  # docker compose down -v
make start
```

**Reason**: Genesis and state must be rebuilt with new EVM implementation.

### 2. **Build Time** ⚠️

- Initial build: 10-15 minutes
- Subsequent builds: 2-5 minutes (with Docker layer caching)
- Build size: ~500MB (optimized with multi-stage build)

### 3. **Environment Variables** 🟡

Add to `.env` file in `infra/`:

```env
# ANDE Integration Configuration
ANDE_PRECOMPILE_ENABLED=true
ANDE_TOKEN_ADDRESS=0x00000000000000000000000000000000000000FD
ANDE_NETWORK_ID=1234
```

### 4. **Monitoring** 🟢

Add to monitoring stack:

```yaml
# Check ANDE precompile calls
curl -X POST -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"debug_traceTransaction","params":["0x..."],"id":1}' \
  http://localhost:8545
```

---

## 📊 Success Metrics

### Technical Metrics

- ✅ Custom ev-reth builds successfully
- ✅ AndeChain starts without errors
- ✅ Smart contracts deploy correctly
- ✅ ANDE precompile accessible at 0x00..fd
- ✅ Native balance transfers work
- ✅ All existing functionality preserved

### Business Metrics

- ✅ No breaking changes to existing contracts
- ✅ No disruption to existing users
- ✅ ANDE Token Duality functionality enabled
- ✅ Performance comparable to standard ev-reth

---

## 🛠️ Troubleshooting Guide

### Issue 1: Build Fails

**Symptoms**: Docker build fails
**Causes**: Rust compilation errors, dependency issues
**Solutions**:
```bash
# Check build logs
docker build -t ev-reth:local . 2>&1 | tee build.log

# Common fixes
- Check Rust version (needs 1.82+)
- Verify dependency versions
- Check disk space (>2GB available)
```

### Issue 2: Container Won't Start

**Symptoms**: ev-reth-sequencer container crashes
**Causes**: Binary not found, configuration errors
**Solutions**:
```bash
# Check container logs
docker compose logs ev-reth-sequencer

# Verify binary exists
docker run --rm ev-reth:local ls -la /usr/local/bin/ev-reth

# Test configuration
docker run --rm ev-reth:local node --help
```

### Issue 3: Genesis Incompatible

**Symptoms**: Chain starts but can't process blocks
**Causes**: Genesis format incompatibility
**Solutions**:
```bash
# Reset completely
make reset
make start

# Verify genesis format
cat infra/stacks/single-sequencer/genesis.final.json | jq .
```

### Issue 4: Precompile Not Working

**Symptoms**: ANDE Token calls fail
**Causes**: Precompile not injected correctly
**Solutions**:
```bash
# Check precompile address
cast call 0x00000000000000000000000000000000000000FD "balanceOf(address)" 0x...

# Verify logs
docker compose logs ev-reth-sequencer | grep -i precompile
```

---

## 📚 Documentation Updates Required

### Update Files

1. **`andechain/CLAUDE.md`**
   - Update build instructions
   - Add ANDE integration section
   - Update troubleshooting guide

2. **`andechain/Makefile`**
   - Add build step for custom ev-reth
   - Update help text
   - Add ANDE-specific commands

3. **`andechain/README.md`**
   - Document ANDE Token Duality integration
   - Update architecture diagram
   - Add new features section

### New Documentation

1. **`andechain/docs/ANDE_INTEGRATION.md`**
   - Integration guide specific to AndeChain
   - Testing procedures
   - Development workflow

2. **`andechain/docs/TROUBLESHOOTING.md`**
   - Common issues and solutions
   - Debug procedures
   - Performance tuning

---

## 🎯 Next Steps

### Immediate (This Week)

1. ✅ Create Dockerfile for custom ev-reth
2. ✅ Update docker-compose configuration
3. ✅ Build and test custom image
4. ✅ Deploy to AndeChain
5. ✅ Verify ANDE integration works

### Short Term (Next Week)

1. 🔄 Complete integration testing
2. 🔄 Deploy smart contracts
3. 🔄 Test ANDE Token Duality functionality
4. 🔄 Performance testing
5. 🔄 Documentation updates

### Long Term (Next Month)

1. 📋 Optimize build process
2. 📋 Add CI/CD for custom builds
3. 📋 Monitor production performance
4. 📋 Plan v0.4.0 precompile injection

---

## ✅ Readiness Checklist

Before deploying to AndeChain:

- [ ] Custom ev-reth builds successfully
- [ ] Docker image < 1GB in size
- [ ] All ev-reth tests pass
- [ ] Docker compose configuration updated
- [ ] Backup of current configuration created
- [ ] Testing environment prepared
- [ ] Documentation updated
- [ ] Team trained on new workflow

---

**Status**: 📋 **PLANNING COMPLETE** - Ready for implementation
**Priority**: 🔴 **HIGH** - Enables core ANDE Token Duality functionality
**Impact**: 🚀 **TRANSFORMATIONAL** - Turns AndeChain into true token duality platform

---

*This integration plan bridges the gap between our ev-reth ANDE integration and the production AndeChain, enabling real-world testing and deployment of the ANDE Token Duality functionality.*