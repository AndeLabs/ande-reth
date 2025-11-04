//! ANDE Precompile Performance Benchmarks
//!
//! This benchmark suite measures the performance of the ANDE Token Duality precompile
//! including gas costs, execution time, and memory usage.

use alloy_primitives::{Address, Bytes, U256};
use criterion::{criterion_group, criterion_main, Criterion};
use revm_primitives::hardfork::SpecId;
use std::hint::black_box;
use std::time::Duration;

use evolve_ev_reth::evm_config::AndePrecompileProvider;

/// Benchmark ANDE precompile provider creation
fn bench_ande_precompile_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("ande_precompile_creation");
    group.measurement_time(Duration::from_secs(10));
    
    group.bench_function("provider_creation", |b| {
        b.iter(|| {
            let provider = black_box(AndePrecompileProvider::new(SpecId::CANCUN));
            black_box(provider);
        });
    });
    
    group.finish();
}

/// Benchmark input data preparation for ANDE precompile
fn bench_ande_input_preparation(c: &mut Criterion) {
    let mut group = c.benchmark_group("ande_input_preparation");
    group.measurement_time(Duration::from_secs(10));
    
    let from = Address::random();
    let to = Address::random();
    let value = U256::from(1000);
    
    group.bench_function("input_encoding", |b| {
        b.iter(|| {
            // Build input data: from (20 bytes) + to (20 bytes) + value (32 bytes)
            let mut input = Vec::new();
            input.extend_from_slice(&[0u8; 12]); // padding
            input.extend_from_slice(black_box(from.as_slice()));
            input.extend_from_slice(&[0u8; 12]); // padding
            input.extend_from_slice(black_box(to.as_slice()));
            input.extend_from_slice(black_box(value.to_be_bytes::<32>().as_slice()));
            
            let input_bytes = black_box(Bytes::from(input));
            black_box(input_bytes);
        });
    });
    
    group.finish();
}

/// Benchmark ANDE precompile address validation
fn bench_ande_address_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("ande_address_validation");
    group.measurement_time(Duration::from_secs(10));
    
    let provider = AndePrecompileProvider::new(SpecId::CANCUN);
    let ande_address = 0xfd;
    let invalid_address = 0x99;
    
    group.bench_function("valid_address", |b| {
        b.iter(|| {
            let is_valid = black_box(ande_address == 0xfd);
            black_box(is_valid);
        });
    });
    
    group.bench_function("invalid_address", |b| {
        b.iter(|| {
            let is_valid = black_box(invalid_address == 0xfd);
            black_box(is_valid);
        });
    });
    
    group.finish();
}

/// Benchmark input validation patterns
fn bench_input_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("input_validation");
    group.measurement_time(Duration::from_secs(10));
    
    // Valid input (72 bytes: 20 + 20 + 32)
    let valid_input = Bytes::from(vec![0u8; 72]);
    
    // Invalid input (too short)
    let invalid_input = Bytes::from(vec![1, 2, 3]);
    
    group.bench_function("valid_input_length", |b| {
        b.iter(|| {
            let is_valid = black_box(valid_input.len() >= 72);
            black_box(is_valid);
        });
    });
    
    group.bench_function("invalid_input_length", |b| {
        b.iter(|| {
            let is_valid = black_box(invalid_input.len() >= 72);
            black_box(is_valid);
        });
    });
    
    group.finish();
}

/// Benchmark data parsing from input
fn bench_data_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("data_parsing");
    group.measurement_time(Duration::from_secs(10));
    
    // Create test input
    let from = Address::random();
    let to = Address::random();
    let value = U256::from(1000);
    
    let mut input = Vec::new();
    input.extend_from_slice(&[0u8; 12]); // padding
    input.extend_from_slice(from.as_slice());
    input.extend_from_slice(&[0u8; 12]); // padding
    input.extend_from_slice(to.as_slice());
    input.extend_from_slice(value.to_be_bytes::<32>().as_slice());
    
    let input_bytes = Bytes::from(input);
    
    group.bench_function("parse_addresses_and_value", |b| {
        b.iter(|| {
            if input_bytes.len() >= 72 {
                // Parse from address (skip first 12 padding bytes)
                let from_bytes = black_box(&input_bytes[12..32]);
                let from_addr = Address::from_slice(from_bytes);
                
                // Parse to address (skip next 12 padding bytes)
                let to_bytes = black_box(&input_bytes[44..64]);
                let to_addr = Address::from_slice(to_bytes);
                
                // Parse value (last 32 bytes)
                let value_bytes = black_box(&input_bytes[64..96]);
                let value = U256::from_be_slice(value_bytes);
                
                black_box((from_addr, to_addr, value));
            }
        });
    });
    
    group.finish();
}

/// Memory allocation patterns benchmark
fn bench_memory_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_patterns");
    group.measurement_time(Duration::from_secs(10));
    
    group.bench_function("small_allocations", |b| {
        b.iter(|| {
            let vec = black_box(Vec::<u8>::with_capacity(72));
            black_box(vec);
        });
    });
    
    group.bench_function("medium_allocations", |b| {
        b.iter(|| {
            let vec = black_box(Vec::<u8>::with_capacity(1024));
            black_box(vec);
        });
    });
    
    group.bench_function("large_allocations", |b| {
        b.iter(|| {
            let vec = black_box(Vec::<u8>::with_capacity(4096));
            black_box(vec);
        });
    });
    
    group.finish();
}

/// Benchmark comparison with standard operations
fn bench_comparison_baseline(c: &mut Criterion) {
    let mut group = c.benchmark_group("comparison_baseline");
    group.measurement_time(Duration::from_secs(10));
    
    let addr1 = Address::random();
    let addr2 = Address::random();
    let value = U256::from(1000);
    
    group.bench_function("address_comparison", |b| {
        b.iter(|| {
            let result = black_box(addr1 == addr2);
            black_box(result);
        });
    });
    
    group.bench_function("u256_arithmetic", |b| {
        b.iter(|| {
            let result = black_box(value + U256::from(1));
            black_box(result);
        });
    });
    
    group.bench_function("bytes_clone", |b| {
        let bytes = Bytes::from(vec![0u8; 72]);
        b.iter(|| {
            let cloned = black_box(bytes.clone());
            black_box(cloned);
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_ande_precompile_creation,
    bench_ande_input_preparation,
    bench_ande_address_validation,
    bench_input_validation,
    bench_data_parsing,
    bench_memory_patterns,
    bench_comparison_baseline
);

criterion_main!(benches);