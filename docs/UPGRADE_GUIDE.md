# ANDE Token Duality - Upgrade Guide

**Purpose**: Guide for future engineers upgrading the ANDE Token Duality integration
**Target**: ev-reth with reth v1.7.0+
**Current Version**: v0.3.0 (Complete Integration)
**Next Target**: v0.4.0 (Precompile Injection)

---

## 🎯 Upgrade Objectives

This guide provides step-by-step instructions for upgrading the ANDE Token Duality integration in ev-reth. It addresses common upgrade scenarios and provides best practices for maintaining compatibility while implementing new features.

---

## 📋 Pre-Upgrade Checklist

### Environment Preparation

1. **Backup Current Implementation**
   ```bash
   # Create backup branch
   git checkout -b backup/v0.3.0-working
   git add .
   git commit -m "backup: working v0.3.0 implementation"

   # Return to main branch
   git checkout main
   ```

2. **Verify Current State**
   ```bash
   # Ensure all tests pass
   cargo test --package ev-tests

   # Verify clean build
   cargo build --workspace

   # Check for any existing issues
   cargo check --workspace
   ```

3. **Documentation Review**
   - Read `docs/ANDE_INTEGRATION_GUIDE.md`
   - Review `docs/CHANGELOG.md`
   - Understand current architecture

### Dependencies Check

1. **Reth Version Compatibility**
   ```bash
   # Check current reth version
   grep -r "reth.*=" Cargo.toml
   grep -r "tag.*=.*v1.7.0" Cargo.toml
   ```

2. **Alloy Package Versions**
   ```bash
   # Check critical dependency versions
   grep "alloy-consensus" Cargo.toml  # Should be 1.0.38
   grep "revm" Cargo.toml            # Should be 29.0.0
   ```

---

## 🔄 Upgrade Scenarios

### Scenario A: Reth Version Upgrade (Minor Versions)

**When**: Upgrading within reth v1.7.x releases
**Risk**: Low
**Expected Changes**: API compatibility, minor deprecations

#### Steps

1. **Update Dependency Versions**
   ```toml
   # In workspace Cargo.toml
   reth-* = { git = "https://github.com/paradigmxyz/reth.git", tag = "v1.7.1" }
   ```

2. **Test Compilation**
   ```bash
   cargo check --workspace
   ```

3. **Fix Compilation Errors**
   - Update API calls if needed
   - Handle deprecated functions
   - Update import statements

4. **Run Full Test Suite**
   ```bash
   cargo test --package ev-tests
   ```

5. **Update Documentation**
   - Update version numbers in `CHANGELOG.md`
   - Document any API changes in `ANDE_INTEGRATION_GUIDE.md`

#### Common Issues & Solutions

- **API Deprecation**: Replace deprecated methods with new equivalents
- **Import Changes**: Update import paths for reorganized modules
- **Type Changes**: Adapt to new type signatures

---

### Scenario B: Reth Major Version Upgrade (v1.7.0 → v1.8.0+)

**When**: Upgrading to new major reth version
**Risk**: High
**Expected Changes**: Breaking API changes, architecture modifications

#### Steps

1. **Research Breaking Changes**
   ```bash
   # Check reth changelog
   # Read migration guides
   # Identify breaking changes affecting our integration
   ```

2. **Create Upgrade Branch**
   ```bash
   git checkout -b upgrade/reth-v1.8.0
   ```

3. **Update Dependencies**
   ```toml
   # Update all reth dependencies
   reth-* = { git = "https://github.com/paradigmxyz/reth.git", tag = "v1.8.0" }
   ```

4. **Address Breaking Changes**
   - **ConfigureEvm Trait**: Check for trait signature changes
   - **Payload Builder**: Verify builder API compatibility
   - **Precompile Provider**: Ensure trait still exists and works

5. **Modify Integration if Needed**
   ```rust
   // Example: If ConfigureEvm trait changes
   impl reth_evm::ConfigureEvm for AndeEvmConfig {
       // Update method signatures to match new trait
   }
   ```

6. **Update Tests**
   - Adapt test code to new APIs
   - Update imports and type signatures
   - Verify all tests still pass

7. **Performance Validation**
   ```bash
   # Run benchmarks if available
   cargo bench

   # Verify no regressions
   ```

#### Critical Points to Check

- **Trait Implementations**: All `ConfigureEvm` methods must be implemented
- **Type Compatibility**: Ensure all custom types still work
- **Test Coverage**: All existing tests must pass

---

### Scenario C: Implementing Precompile Injection (v0.3.0 → v0.4.0)

**When**: Adding actual precompile execution to EVM
**Risk**: Medium
**Expected Changes**: Type alias → struct wrapper, EVM modification

#### Steps

1. **Design New Architecture**
   ```rust
   // Replace type alias with struct wrapper
   pub struct AndeEvmConfig {
       inner: EthEvmConfig,
       precompile_provider: AndePrecompileProvider,
   }
   ```

2. **Implement ConfigureEvm Trait**
   ```rust
   impl ConfigureEvm for AndeEvmConfig {
       fn evm<'a, DB: reth_revm::database::DB + Send + Sync>(
           &self,
           db: DB,
       ) -> reth_revm::Evm<'a, reth_revm::NoOpInspector, DB> {
           let mut evm = self.inner.evm(db);

           // TODO: Inject AndePrecompileProvider here
           // This requires deep integration with revm internals

           evm
       }

       // Implement all other required trait methods
   }
   ```

3. **Update Factory Function**
   ```rust
   pub fn create_ande_evm_config(chain_spec: Arc<ChainSpec>) -> AndeEvmConfig {
       let inner = EthEvmConfig::new(chain_spec);
       let precompile_provider = AndePrecompileProvider::new();

       AndeEvmConfig {
           inner,
           precompile_provider,
       }
   }
   ```

4. **Research revm Precompile Injection**
   - Study revm's precompile registration API
   - Understand how to modify EVM context
   - Implement safe precompile injection

5. **Update Tests**
   - Test actual precompile execution
   - Verify native balance transfers
   - Test integration with smart contracts

6. **Add Integration Tests**
   ```rust
   #[test]
   fn test_precompile_execution() {
       // Test that precompile actually executes
       // Verify native balance changes
       // Validate transaction processing
   }
   ```

#### Technical Challenges

- **revm Internals**: Understanding revm's precompile system
- **State Modification**: Safe journal.transfer() usage
- **EVM Context**: Proper context setup and cleanup

---

## 🔧 Maintenance Tasks

### Regular Maintenance

1. **Monthly Dependency Updates**
   ```bash
   # Check for security updates
   cargo audit

   # Update minor versions
   cargo update

   # Test compatibility
   cargo test --package ev-tests
   ```

2. **Quarterly Architecture Review**
   - Review integration patterns
   - Check for better implementation approaches
   - Evaluate performance optimizations

3. **Annual Major Upgrade Assessment**
   - Assess need for reth version upgrades
   - Plan for breaking changes
   - Budget development time

### Monitoring

1. **Test Coverage**
   ```bash
   # Run coverage analysis
   cargo tarpaulin --out Html

   # Verify minimum coverage thresholds
   ```

2. **Performance Benchmarks**
   ```bash
   # Run performance tests
   cargo bench

   # Compare with baseline
   ```

3. **Security Audits**
   ```bash
   # Regular security scanning
   cargo audit
   ```

---

## 🚨 Troubleshooting Guide

### Common Upgrade Issues

#### Issue 1: Compilation Errors After Dependency Update

**Symptoms**: Build fails with type errors
**Causes**: API changes in dependencies
**Solutions**:
1. Check error messages for deprecated APIs
2. Update method calls to new signatures
3. Review dependency documentation

#### Issue 2: Test Failures

**Symptoms**: Tests fail after upgrade
**Causes**: Behavior changes in dependencies
**Solutions**:
1. Run tests with verbose output
2. Check for assertion failures
3. Update test expectations if behavior changed intentionally

#### Issue 3: Performance Regression

**Symptoms**: Slower execution after upgrade
**Causes**: Inefficient new APIs or changes
**Solutions**:
1. Run benchmarks before and after
2. Profile to identify bottlenecks
3. Optimize critical paths

#### Issue 4: Integration Incompatibility

**Symptoms**: Payload builder or EVM issues
**Causes**: Breaking changes in core APIs
**Solutions**:
1. Verify trait implementations
2. Check method signatures
3. Update integration code

### Debugging Techniques

1. **Verbose Logging**
   ```bash
   RUST_LOG=debug cargo test --package ev-tests
   ```

2. **Step-by-Step Execution**
   ```bash
   # Use debugger
   rust-gdb target/debug/deps/ev_tests-*
   ```

3. **Isolate Issues**
   ```bash
   # Test individual components
   cargo test test_ande_evm_config_integration
   ```

---

## 📚 Additional Resources

### Documentation
- **Primary**: `docs/ANDE_INTEGRATION_GUIDE.md`
- **Changes**: `docs/CHANGELOG.md`
- **Status**: `docs/V0.3.0_INTEGRATION_STATUS.md`

### External Resources
- **Reth Documentation**: https://docs.reth.com/
- **Rust Guidelines**: https://rust-lang.github.io/api-guidelines/
- **Testing Best Practices**: https://doc.rust-lang.org/book/testing.html

### Community
- **Reth Discord**: For community support
- **Rust Users Forum**: For general Rust questions
- **GitHub Issues**: For bug reports and feature requests

---

## 🔄 Rollback Procedures

### Emergency Rollback

1. **Identify Broken Version**
   ```bash
   git log --oneline -10
   ```

2. **Revert to Working Version**
   ```bash
   git checkout <working-commit-hash>
   ```

3. **Verify Working State**
   ```bash
   cargo test --package ev-tests
   ```

4. **Document Issues**
   - Record what broke
   - Note root causes
   - Plan fix approach

### Partial Rollback

1. **Isolate Problematic Changes**
   ```bash
   git bisect start
   git bisect bad HEAD
   git bisect good <working-commit>
   ```

2. **Revert Specific Files**
   ```bash
   git checkout <working-commit> -- <problematic-file>
   ```

---

## ✅ Upgrade Success Criteria

An upgrade is considered successful when:

1. **All Tests Pass**: No test regressions
2. **Clean Build**: No compilation errors or warnings
3. **Performance Maintained**: No significant performance degradation
4. **Documentation Updated**: All documentation reflects changes
5. **Backward Compatibility**: Existing functionality preserved

---

## 🎯 Next Major Upgrade (v0.4.0)

**Target**: Precompile Injection Implementation
**Timeline**: TBD based on business requirements
**Priority**: High (enables actual token duality functionality)

**Key Objectives**:
- Replace type alias with struct wrapper
- Inject `AndePrecompileProvider` into EVM execution
- Enable native balance transfers during transaction processing
- Add comprehensive integration testing

---

*This upgrade guide should be updated with each new version to reflect current best practices and known issues.*