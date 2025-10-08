# AndeChain New Workflow - ANDE Token Duality

**Purpose**: Complete guide to the new development workflow after ANDE Token Duality integration
**Date**: 2025-10-08
**Status**: ✅ **READY FOR PRODUCTION**

---

## 🔄 Workflow Transformation

### Before ANDE Integration (Old Workflow)

```bash
cd andechain
make start                    # Uses standard ev-reth
make deploy-ecosystem         # Deploys contracts
make test                    # Tests existing functionality
```

### After ANDE Integration (New Workflow)

```bash
cd andechain
make start-ande              # Uses custom ev-reth with ANDE
make deploy-ecosystem         # Deploys contracts (unchanged)
make test-ande               # Tests ANDE integration
make health-ande             # Health check for ANDE
```

---

## 🚀 New Development Commands

### Core ANDE Commands

```bash
# Build custom ev-reth with ANDE integration
make build-ev-reth

# Start AndeChain with ANDE integration
make start-ande

# Reset and start fresh with ANDE
make reset-ande

# Test ANDE integration functionality
make test-ande

# Quick health check for ANDE
make health-ande

# Show ANDE integration information
make info-ande
```

### Standard Commands (Still Available)

```bash
# Standard start (auto-detects ANDE)
make start

# Deploy ecosystem contracts
make deploy-ecosystem

# Standard tests
make test

# Stop infrastructure
make stop

# Complete reset
make reset
```

---

## 📋 Daily Development Workflow

### Morning Setup

```bash
# 1. Navigate to project
cd andechain

# 2. Check current status
make health-ande

# 3. If not running, start with ANDE
make start-ande

# 4. Wait for services (60 seconds)
sleep 60

# 5. Verify blockchain is running
curl -X POST -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' \
  http://localhost:8545
```

### Development Iteration

```bash
# 1. Make changes to smart contracts
# (Edit .sol files in contracts/)

# 2. Test contracts
cd contracts
forge test -vv

# 3. If tests pass, deploy
cd ..
make deploy-ecosystem

# 4. Test ANDE integration
make test-ande

# 5. Health check
make health-ande
```

### ANDE-Specific Development

```bash
# 1. Build/rebuild custom ev-reth (if needed)
cd ev-reth
# Make changes to ANDE integration code
./scripts/build-ev-reth.sh

# 2. Restart AndeChain with new build
cd ../andechain
make reset-ande

# 3. Deploy ANDEToken contracts
cd contracts
export PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
forge script script/DeployAndTestVeANDE.s.sol --rpc-url local --broadcast --legacy

# 4. Test ANDE functionality
cd ..
make test-ande
```

---

## 🔍 Testing Workflow

### Standard Testing

```bash
# Test existing contracts
cd contracts
forge test -vv

# Gas report
forge test --gas-report

# Coverage
forge coverage
```

### ANDE Integration Testing

```bash
# Test ANDE precompile integration
make test-ande

# Manual precompile testing
cast call 0x00000000000000000000000000000000000000FD "balanceOf(address)" 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 --rpc-url local

# Test ANDEToken contract (when deployed)
cast call <ANDE_TOKEN_ADDRESS> "transfer(address,uint256)" 0x... 1000000000000000000 --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 --rpc-url local
```

### End-to-End Testing

```bash
# 1. Start with ANDE
make start-ande

# 2. Deploy all contracts
make deploy-ecosystem

# 3. Test standard functionality
make test

# 4. Test ANDE functionality
make test-ande

# 5. Verify all services
make health-ande
```

---

## 🛠️ Troubleshooting Workflow

### Quick Health Check

```bash
# Always start here
make health-ande
```

### Common Issues & Solutions

#### Issue 1: Blockchain Not Responding

```bash
# Check container status
cd infra && docker compose ps

# Check logs
docker compose logs ev-reth-sequencer | tail -20

# Restart if needed
docker compose restart ev-reth-sequencer

# Complete reset if persistent issues
cd .. && make reset-ande
```

#### Issue 2: Build Failures

```bash
# Check ev-reth build
cd ../ev-reth
./scripts/build-ev-reth.sh

# Common fixes
- Check Docker is running
- Verify sufficient disk space (>2GB)
- Check network connection for dependencies

# Rebuild if needed
docker build -t ev-reth:local . --no-cache
```

#### Issue 3: Precompile Not Working

```bash
# Verify precompile address
cast code 0x00000000000000000000000000000000000000FD --rpc-url local

# Expected: 0x (not injected yet in v0.3.0)
# If different, check implementation

# Test accessibility
cast call 0x00000000000000000000000000000000000000FD "balanceOf(address)" 0x... --rpc-url local
```

#### Issue 4: Contract Deployment Failures

```bash
# Check blockchain is running
curl -X POST -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' \
  http://localhost:8545

# Check account balance
cast balance 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 --rpc-url local

# Redeploy contracts
cd contracts
export PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
forge script script/DeployEcosystem.s.sol:DeployEcosystem --rpc-url local --broadcast --legacy
```

---

## 📊 Monitoring & Verification

### Daily Health Check

```bash
# Morning check
make health-ande

# Weekly full check
make test-ande
make test
```

### Performance Monitoring

```bash
# Check container resource usage
docker stats ev-reth-sequencer

# Check logs for errors
cd infra && docker compose logs ev-reth-sequencer | grep -i error

# Check blockchain sync
curl -X POST -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"eth_syncing","params":[],"id":1}' \
  http://localhost:8545
```

### Development Metrics

```bash
# Track number of blocks
curl -s -X POST -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' \
  http://localhost:8545 | jq '.result'

# Track gas usage
forge test --gas-report

# Track test coverage
forge coverage
```

---

## 🔄 Git Workflow Integration

### Feature Development with ANDE

```bash
# 1. Create feature branch
git checkout develop
git pull origin develop
git checkout -b feature/ande-contract-integration

# 2. Develop with ANDE
make start-ande
# Make changes to contracts
make test-ande

# 3. Commit changes
git add .
git commit -m "feat(ande): integrate ANDEToken with precompile"

# 4. Test and deploy
make deploy-ecosystem

# 5. Push and create PR
git push origin feature/ande-contract-integration
```

### Infrastructure Changes

```bash
# 1. Create branch for infrastructure changes
git checkout -b feature/ande-infrastructure-update

# 2. Make changes to Docker configuration
# Edit files in infra/

# 3. Test changes
make reset-ande
make health-ande

# 4. Document changes
# Update CLAUDE.md, this guide, etc.

# 5. Commit and push
git add .
git commit -m "feat(infra): update docker config for ANDE integration"
git push origin feature/ande-infrastructure-update
```

---

## 📚 Documentation Updates

### When to Update Documentation

1. **After ANDE Code Changes**: Update `docs/ANDE_INTEGRATION_GUIDE.md`
2. **After Workflow Changes**: Update this guide
3. **After New Features**: Update `CLAUDE.md`
4. **After Bug Fixes**: Update `docs/TROUBLESHOOTING.md`

### Documentation Workflow

```bash
# 1. Make code changes
# (Write code, test, etc.)

# 2. Update documentation
vim docs/ANDE_INTEGRATION_GUIDE.md
vim CLAUDE.md
vim docs/ANDECHAIN_NEW_WORKFLOW.md

# 3. Test documentation
# Follow your own instructions

# 4. Commit both code and docs
git add .
git commit -m "feat: add new ANDE feature with documentation"
```

---

## 🎯 Best Practices

### Development Best Practices

1. **Always Start with Health Check**
   ```bash
   make health-ande
   ```

2. **Test Before Deploying**
   ```bash
   make test-ande && make deploy-ecosystem
   ```

3. **Use Meaningful Commit Messages**
   ```bash
   git commit -m "feat(ande): implement token transfer with precompile"
   ```

4. **Document New Features**
   - Update relevant documentation
   - Add examples to this guide
   - Update API references

### Production Best Practices

1. **Monitor Chain Health**
   ```bash
   # Add to crontab for daily checks
   0 9 * * * cd /path/to/andechain && make health-ande >> /var/log/ande-health.log
   ```

2. **Backup Configuration**
   ```bash
   #定期备份重要配置
   cp infra/stacks/single-sequencer/docker-compose.da.local.yml backups/
   ```

3. **Version Control**
   ```bash
   # Tag important deployments
   git tag -a v0.3.0-ande-deployment -m "ANDE integration deployment"
   ```

---

## 🚀 Advanced Workflow Scenarios

### Scenario 1: Multiple Developers

```bash
# Developer A starts with ANDE
make start-ande

# Developer B joins
git pull origin main
make health-ande  # Check if running
# If not running, start with standard start
make start
```

### Scenario 2: CI/CD Integration

```bash
# In CI/CD pipeline
- name: Build ANDE ev-reth
  run: |
    cd ev-reth
    ./scripts/build-ev-reth.sh

- name: Start AndeChain
  run: |
    cd andechain
    make start-ande

- name: Test ANDE Integration
  run: |
    cd andechain
    make test-ande
```

### Scenario 3: Production Deployment

```bash
# 1. Staging testing
make reset-ande
make deploy-ecosystem
make test-ande

# 2. Production deployment
# (Similar commands on production server)

# 3. Monitoring
make health-ande
# Set up monitoring alerts
```

---

## ✅ Quick Reference

### Essential Commands

```bash
make start-ande      # Start with ANDE
make health-ande    # Check ANDE status
make test-ande      # Test ANDE functionality
make info-ande      # Show ANDE info
make reset-ande     # Reset with ANDE
```

### Precompile Address

```
ANDE Precompile: 0x00000000000000000000000000000000000000FD
```

### Endpoints

```bash
RPC:          http://localhost:8545
Explorer:     http://localhost:4000
Metrics:      http://localhost:9001
```

### Troubleshooting

```bash
# Always start here
make health-ande

# Check logs
cd infra && docker compose logs ev-reth-sequencer

# Reset if needed
make reset-ande
```

---

## 🎉 Conclusion

The new workflow provides a seamless transition to ANDE Token Duality integration while maintaining all existing functionality. The key improvements are:

1. **Automated ANDE Integration**: One-command startup with ANDE
2. **Comprehensive Testing**: Built-in ANDE functionality tests
3. **Health Monitoring**: Real-time status checks
4. **Backward Compatibility**: All existing commands still work
5. **Future-Ready**: Prepared for v0.4.0 precompile injection

The workflow is designed to be intuitive for existing developers while providing powerful new capabilities for ANDE Token Duality development.

---

**Status**: ✅ **WORKFLOW READY FOR PRODUCTION**
**Team Training**: Required
**Documentation**: Complete
**Next Step**: Team adoption and feedback collection

---

*This workflow guide serves as the definitive reference for day-to-day development with ANDE Token Duality integration in AndeChain.*