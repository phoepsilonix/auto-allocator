//! Auto-Allocator Performance Benchmarks
//!
//! This benchmark suite provides comprehensive memory allocator performance evaluation, including:
//!
//! ## Test Scenarios
//!
//! 1. **Basic Allocation Tests** - Single allocation/deallocation of different sizes
//! 2. **Batch Allocation Tests** - Bulk operations for small and large memory blocks
//! 3. **Real Application Simulation** - String operations, vector expansion and other real-world scenarios
//! 4. **Memory Fragmentation Tests** - Mixed-size allocation simulating memory fragmentation scenarios
//! 5. **Concurrent Allocation Tests** - Allocator performance in multi-threaded environments
//!
//! ## Usage
//!
//! ```bash
//! # Run complete benchmark tests (automatically generates HTML reports)
//! cargo bench
//!
//! # Run specific benchmark tests
//! cargo bench --bench allocator_benchmark
//!
//! # View allocator performance in different modes
//! cargo bench  # Debug mode uses system allocator
//! cargo bench --release  # Release mode automatically selects optimal allocator
//!
//! # View detailed reports
//! open target/criterion/report/index.html
//! ```
//!
//! ## Interpreting Results
//!
//! - **time** - Operation duration (lower is better)
//! - **throughput** - Throughput (higher is better)
//! - **Elements/sec** - Number of elements processed per second
//! - **slope** - Performance trend (stable is best)

use criterion::{
    black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput,
};
use std::sync::Arc;
use std::thread;

// Import auto_allocator to ensure using the selected allocator
#[allow(clippy::single_component_path_imports)]
use auto_allocator;

/// Basic memory allocation performance tests
///
/// Tests allocation/deallocation performance for different memory block sizes,
/// which are fundamental allocator performance metrics
fn bench_basic_allocation(c: &mut Criterion) {
    // Get and display allocator information
    let info = auto_allocator::get_allocator_info();

    // Only show information on first run
    static SHOWN: std::sync::Once = std::sync::Once::new();
    SHOWN.call_once(|| {
        println!("Benchmarking with allocator: {:?}", info.allocator_type);
        println!("Selection reason: {}", info.reason);
        println!(
            "System: {} cores, {} memory",
            info.system_info.cpu_cores,
            auto_allocator::format_memory_size(info.system_info.total_memory_bytes)
        );
    });

    let mut group = c.benchmark_group("basic_allocation");

    // Test different size allocations from 16B to 16KB
    for size in [16, 64, 256, 1024, 4096, 16384].iter() {
        group.throughput(Throughput::Bytes(*size as u64));

        // Single allocation/deallocation performance (using Box for safety)
        group.bench_with_input(BenchmarkId::new("raw_alloc", size), size, |b, &size| {
            b.iter(|| {
                let data = vec![0u8; size].into_boxed_slice();
                black_box(data);
            });
        });

        // Vec allocation performance (closer to real usage)
        group.bench_with_input(BenchmarkId::new("vec_alloc", size), size, |b, &size| {
            b.iter(|| {
                let vec: Vec<u8> = vec![0; size];
                black_box(vec);
            });
        });
    }

    group.finish();
}

/// Batch allocation performance tests
///
/// Simulates common bulk memory allocation scenarios in applications
fn bench_batch_allocation(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_allocation");

    // Small block batch allocation (simulating frequent small object creation)
    group.bench_function("small_batch_1000", |b| {
        b.iter_batched(
            Vec::new,
            |mut ptrs| {
                // Allocation
                for _ in 0..1000 {
                    let vec: Vec<u8> = vec![0; 64];
                    ptrs.push(vec);
                }
                // Deallocation
                ptrs.clear();
            },
            BatchSize::SmallInput,
        );
    });

    // Large block batch allocation (simulating buffer or data structure allocation)
    group.bench_function("large_batch_100", |b| {
        b.iter_batched(
            Vec::new,
            |mut ptrs| {
                // Allocation
                for _ in 0..100 {
                    let vec: Vec<u8> = vec![0; 8192];
                    ptrs.push(vec);
                }
                // Deallocation
                ptrs.clear();
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

/// Real application scenario simulation tests
///
/// Simulates memory usage patterns in real applications
fn bench_real_world_scenarios(c: &mut Criterion) {
    let mut group = c.benchmark_group("real_world");

    // String processing scenario
    group.bench_function("string_processing", |b| {
        b.iter(|| {
            // Create multiple strings
            let mut strings = Vec::new();
            for i in 0..100 {
                strings.push(format!("String number {}", i));
            }

            // String concatenation operations
            let mut result = String::new();
            for s in strings {
                result.push_str(&s);
                result.push(' ');
            }
            black_box(result);
        });
    });

    // Data structure operations scenario
    group.bench_function("data_structures", |b| {
        b.iter(|| {
            // Dynamic array expansion
            let mut vec = Vec::new();
            for i in 0..1000 {
                vec.push(i);
            }

            // Hash table operations
            let mut map = std::collections::HashMap::new();
            for i in 0..100 {
                map.insert(format!("key_{}", i), i);
            }

            black_box((vec, map));
        });
    });

    // JSON serialization scenario (simulating web applications)
    group.bench_function("json_like_serialization", |b| {
        b.iter(|| {
            let mut result = String::with_capacity(10000);
            result.push('{');

            for i in 0..100 {
                if i > 0 {
                    result.push(',');
                }
                result.push_str(&format!("\"field_{}\": \"value_{}\"", i, i * 2));
            }

            result.push('}');
            black_box(result);
        });
    });

    group.finish();
}

/// Memory fragmentation tests
///
/// Tests allocator performance under memory fragmentation scenarios
fn bench_fragmentation(c: &mut Criterion) {
    let mut group = c.benchmark_group("fragmentation");

    group.bench_function("mixed_size_fragmentation", |b| {
        b.iter(|| {
            let mut small_ptrs = Vec::new();
            let mut large_ptrs = Vec::new();

            // Phase 1: Alternately allocate small and large memory blocks
            for i in 0..100 {
                if i % 2 == 0 {
                    small_ptrs.push(vec![0u8; 64]);
                } else {
                    large_ptrs.push(vec![0u8; 4096]);
                }
            }

            // Phase 2: Release half of the small blocks (create fragmentation)
            for i in (0..small_ptrs.len()).step_by(2) {
                small_ptrs[i].clear();
            }

            // Phase 3: Try to allocate medium-sized blocks (test defragmentation ability)
            let mut medium_ptrs = Vec::new();
            for _ in 0..50 {
                medium_ptrs.push(vec![0u8; 1024]);
            }

            // Cleanup: Release all remaining memory
            small_ptrs.clear();
            large_ptrs.clear();
            medium_ptrs.clear();
        });
    });

    group.finish();
}

/// Multi-threaded concurrent allocation tests
///
/// Tests allocator performance and contention in multi-threaded environments
fn bench_concurrent_allocation(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent");

    // Test concurrent performance with different thread counts
    for thread_count in [2, 4, 8].iter() {
        group.bench_with_input(
            BenchmarkId::new("concurrent_alloc", thread_count),
            thread_count,
            |b, &thread_count| {
                b.iter(|| {
                    let barrier = Arc::new(std::sync::Barrier::new(thread_count));
                    let handles: Vec<_> = (0..thread_count)
                        .map(|_| {
                            let barrier = Arc::clone(&barrier);
                            thread::spawn(move || {
                                // Wait for all threads to be ready
                                barrier.wait();

                                let mut ptrs = Vec::new();
                                // Each thread allocates 250 small blocks
                                for _ in 0..250 {
                                    ptrs.push(vec![0u8; 128]);
                                }

                                // Release all memory
                                ptrs.clear();
                            })
                        })
                        .collect();

                    // Wait for all threads to complete
                    for handle in handles {
                        handle.join().unwrap();
                    }
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_basic_allocation,
    bench_batch_allocation,
    bench_real_world_scenarios,
    bench_fragmentation,
    bench_concurrent_allocation
);
criterion_main!(benches);
