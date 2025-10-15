//! ANDE Precompile Performance Benchmarks
//!
//! This benchmark suite measures the performance of the ANDE Token Duality precompile
//! including gas costs, execution time, and memory usage.

use alloy_primitives::{Address, Bytes, U256};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use revm::{
    handler::EthPrecompiles,
    interpreter::{Gas, InstructionResult, InterpreterResult},
    precompile::Precompiles,
    primitives::hardfork::SpecId,
};
use std::time::Duration;

use evolve_ev_reth::evm_config::AndePrecompileProvider;

/// Benchmark ANDE precompile execution with valid transfer
fn bench_ande_precompile_transfer(c: &mut Criterion) {
    let mut group = c.benchmark_group("ande_precompile_transfer");
    group.measurement_time(Duration::from_secs(10));
    
    let precompile_provider = AndePrecompileProvider::new();
    let precompiles = precompile_provider.get_precompiles(SpecId::LATEST);
    
    // Test data for a valid transfer
    let from = Address::random();
    let to = Address::random();
    let value = U256::from(1000);
    
    // Build input data: from (20 bytes) + to (20 bytes) + value (32 bytes)
    let mut input = Vec::new();
    input.extend_from_slice(&[0u8; 12]); // padding
    input.extend_from_slice(from.as_slice());
    input.extend_from_slice(&[0u8; 12]); // padding
    input.extend_from_slice(to.as_slice());
    input.extend_from_slice(value.to_be_bytes::<32>().as_slice());
    
    let input_bytes = Bytes::from(input);
    
    group.bench_function("valid_transfer", |b| {
        b.iter(|| {
            let result = precompiles.call(
                black_box(0xfd), // ANDE precompile address
                black_box(&input_bytes),
                Gas::new(100_000),
            );
            black_box(result);
        });
    });
    
    group.finish();
}

/// Benchmark ANDE precompile with invalid input (should fail fast)
fn bench_ande_precompile_invalid(c: &mut Criterion) {
    let mut group = c.benchmark_group("ande_precompile_invalid");
    group.measurement_time(Duration::from_secs(10));
    
    let precompile_provider = AndePrecompileProvider::new();
    let precompiles = precompile_provider.get_precompiles(SpecId::LATEST);
    
    // Invalid input (too short)
    let invalid_input = Bytes::from(vec![1, 2, 3]);
    
    group.bench_function("invalid_input", |b| {
        b.iter(|| {
            let result = precompiles.call(
                black_box(0xfd), // ANDE precompile address
                black_box(&invalid_input),
                Gas::new(100_000),
            );
            black_box(result);
        });
    });
    
    group.finish();
}

/// Benchmark gas calculation for ANDE precompile
fn bench_ande_precompile_gas(c: &mut Criterion) {
    let mut group = c.benchmark_group("ande_precompile_gas");
    group.measurement_time(Duration::from_secs(10));
    
    let precompile_provider = AndePrecompileProvider::new();
    
    // Test data
    let input = Bytes::from(vec![0u8; 72]); // Standard input size
    
    group.bench_function("gas_calculation", |b| {
        b.iter(|| {
            let gas = precompile_provider.gas(
                black_box(0xfd), // ANDE precompile address
                black_box(&input),
            );
            black_box(gas);
        });
    });
    
    group.finish();
}

/// Benchmark standard precompile for comparison
fn bench_standard_precompile(c: &mut Criterion) {
    let mut group = c.benchmark_group("standard_precompile");
    group.measurement_time(Duration::from_secs(10));
    
    let precompiles = Precompiles::new(SpecId::LATEST);
    
    // Use ecRecover as comparison
    let input = Bytes::from(vec![0u8; 128]); // Standard ecRecover input size
    
    group.bench_function("ecrecover", |b| {
        b.iter(|| {
            let result = precompiles.call(
                black_box(0x01), // ecRecover address
                black_box(&input),
                Gas::new(100_000),
            );
            black_box(result);
        });
    });
    
    group.finish();
}

/// Memory usage benchmark
fn bench_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");
    group.measurement_time(Duration::from_secs(10));
    
    let precompile_provider = AndePrecompileProvider::new();
    let precompiles = precompile_provider.get_precompiles(SpecId::LATEST);
    
    // Large input to test memory handling
    let large_input = Bytes::from(vec![0u8; 1024]);
    
    group.bench_function("large_input", |b| {
        b.iter(|| {
            let result = precompiles.call(
                black_box(0xfd), // ANDE precompile address
                black_box(&large_input),
                Gas::new(100_000),
            );
            black_box(result);
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_ande_precompile_transfer,
    bench_ande_precompile_invalid,
    bench_ande_precompile_gas,
    bench_standard_precompile,
    bench_memory_usage
);

criterion_main!(benches);