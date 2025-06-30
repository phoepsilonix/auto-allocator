# Auto-Allocator Performance Benchmarks

This directory contains the complete performance benchmark suite for Auto-Allocator, used to validate the intelligent allocator selection strategy based on performance research.

## 🏆 Allocator Performance Priority

Based on performance research from Microsoft and other authoritative institutions, Auto-Allocator selects allocators according to the following priority:

1. **mimalloc** - Preferred allocator for modern platforms, superior multi-threaded performance
2. **system** - Used for debug builds, WASM, mobile platforms, and maximum compatibility  
3. **embedded** - Specialized allocator for resource-constrained embedded environments

## 🚀 Quick Start

### Run Complete Benchmark Suite

```bash
# Run all benchmarks (automatically generates HTML reports)
cargo bench

# Run specific benchmark
cargo bench --bench allocator_benchmark
```

### Verify Performance Selection

```bash
# Debug mode - verify system allocator selection
cargo bench

# Release mode - verify mimalloc selection on modern platforms
cargo bench --release
```

### Generate Detailed Reports

```bash
# Run benchmarks and generate HTML reports
cargo bench --bench allocator_benchmark

# View detailed reports
open target/criterion/report/index.html
```

## 📊 Benchmark Scenarios

### 1. Basic Allocation Tests (`basic_allocation`)

Tests allocation/deallocation performance for different memory block sizes:

- **raw_alloc**: Single allocation-deallocation operations (16B ~ 16KB) using safe Box allocation
- **vec_alloc**: Vec allocation and population operations

**Key Metrics**: Latency, throughput, allocation efficiency

### 2. Batch Allocation Tests (`batch_allocation`)

Simulates bulk memory operations in applications:

- **small_batch_1000**: Batch allocation of 1000 64-byte small blocks
- **large_batch_100**: Batch allocation of 100 8KB large blocks

**Use Cases**: Data structure creation, cache allocation, batch processing operations

### 3. Real-World Application Simulation (`real_world`)

Simulates memory usage patterns in real applications:

- **string_processing**: String creation and concatenation operations
- **data_structures**: Dynamic array and hash table operations  
- **json_like_serialization**: JSON serialization-like memory allocation patterns

**Importance**: Closest to actual application performance

### 4. Memory Fragmentation Tests (`fragmentation`)

Tests allocator performance under memory fragmentation scenarios:

- **mixed_size_fragmentation**: Mixed-size allocation + partial deallocation + reallocation

**Key Role**: Evaluates long-running service memory management capabilities

### 5. Concurrent Allocation Tests (`concurrent`)

Tests allocator performance in multi-threaded environments:

- **concurrent_alloc**: 2/4/8 thread concurrent small block allocation

**Use Cases**: Multi-threaded servers, parallel computing applications

## 📈 Results Interpretation

### Key Metrics

- **Time**: Operation duration (lower is better)
- **Throughput**: Throughput, such as `Elements/sec` or `Bytes/sec` (higher is better)
- **Slope**: Performance trend with input size changes (stable is best)
- **R²**: Goodness of fit, close to 1.0 indicates stable performance

### Auto-Allocator Performance Characteristics

| Mode | Selected Allocator | Small Block Allocation | Large Block Allocation | Concurrent Performance | Memory Fragmentation | Use Case |
|------|-------------------|----------------------|----------------------|----------------------|-------------------|-----------|
| **Debug** | System | Fair | Fair | Fair | Fair | Rapid development |
| **Release (Modern)** | **mimalloc** | **Excellent** | **Excellent** | **Outstanding** | **Excellent** | **General high-performance applications** |
| **WASM** | System | Fair | Fair | Fair | Fair | Web applications |
| **Mobile** | System | Fair | Fair | Fair | Fair | Platform compliance |
| **Embedded** | embedded-alloc | Excellent* | Good* | Limited | Limited | Embedded systems |

*Relative performance in resource-constrained environments

### Performance Analysis Recommendations

1. **Focus on real-world workloads**: `real_world` tests are closest to real usage scenarios
2. **Check concurrent performance**: If the application is multi-threaded, focus on `concurrent` results
3. **Consider memory fragmentation**: Long-running services should pay attention to `fragmentation` test results
4. **Compare different modes**: Compare performance differences between Debug and Release modes

## 🔧 Runtime Environment Configuration

### Recommended Test Environment

- **Hardware**: CPU ≥ 4 cores, Memory ≥ 8GB
- **System**: Relatively idle machine, avoid other high-load processes
- **Compilation**: Use Release mode for performance benchmarks

### Performance in Different Hardware Environments

#### Modern Platforms (Windows/macOS/Linux)
```bash
# Expected to select mimalloc (performance research-based preference)
cargo bench --release
```

#### Mobile Platforms (Android/iOS)
```bash
# Expected to select system allocator (platform compliance)
cargo bench --release
```

#### Embedded Systems
```bash
# Expected to select embedded allocator (requires embedded targets)
cargo bench --target thumbv7em-none-eabi --release
```

## 📁 Benchmark Output

### Report File Structure

```
target/criterion/
├── report/
│   └── index.html              # Main report page
├── basic_allocation/
│   ├── raw_alloc/
│   │   ├── 16/
│   │   ├── 64/
│   │   └── ...
│   └── vec_alloc/
├── batch_allocation/
│   ├── small_batch_1000/
│   └── large_batch_100/
├── real_world/
│   ├── string_processing/
│   ├── data_structures/
│   └── json_like_serialization/
├── fragmentation/
│   └── mixed_size_fragmentation/
└── concurrent/
    └── concurrent_alloc/
```

### View Reports

```bash
# Open main report in browser
open target/criterion/report/index.html

# macOS users
open target/criterion/report/index.html

# Linux users  
xdg-open target/criterion/report/index.html

# Windows users
start target/criterion/report/index.html
```

## 🎯 Real Application Performance Comparison

### Web Server Scenarios

```bash
# Simulate high-concurrency web server workloads
cargo bench --release --bench allocator_benchmark real_world/json_like_serialization
cargo bench --release --bench allocator_benchmark concurrent
```

### Data Processing Applications

```bash
# Simulate extensive data structure operations
cargo bench --release --bench allocator_benchmark real_world/data_structures
cargo bench --release --bench allocator_benchmark batch_allocation
```

### Embedded Systems

```bash
# Simulate resource-constrained environments (requires embedded targets)
cargo bench --target thumbv7em-none-eabi --bench allocator_benchmark basic_allocation
```

## 🔬 Advanced Analysis

### Performance Regression Detection

```bash
# Save current version as baseline
cargo bench --bench allocator_benchmark --save-baseline main

# Compare performance after code changes
cargo bench --bench allocator_benchmark --baseline main
```

### Statistical Analysis

```bash
# Generate detailed statistical information
cargo bench --bench allocator_benchmark -- --verbose

# Custom confidence intervals
cargo bench --bench allocator_benchmark -- --confidence-level 0.99
```

### Memory Usage Monitoring

```bash
# Use valgrind to monitor memory usage (Linux)
valgrind --tool=massif cargo bench --release --bench allocator_benchmark

# Use heaptrack for monitoring (Linux)
heaptrack cargo bench --release --bench allocator_benchmark
```

## 🤝 Custom Benchmarks

### Adding New Test Scenarios

1. **Add function in `allocator_benchmark.rs`**:

```rust
fn my_custom_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("my_custom_test");
    
    group.bench_function("my_test", |b| {
        b.iter(|| {
            // Your test code
            black_box(my_allocation_test());
        });
    });
    
    group.finish();
}
```

2. **Add to test group**:

```rust
criterion_group!(
    benches,
    bench_basic_allocation,
    bench_batch_allocation,
    bench_real_world_scenarios,
    bench_fragmentation,
    bench_concurrent_allocation,
    my_custom_benchmark  // Add here
);
```

### Testing Best Practices

- **Use `black_box()`** to prevent compiler optimizations
- **Reasonable test scale** to avoid excessively long test times
- **Real scenario simulation** to ensure tests have practical meaning
- **Clear naming** for easy understanding of test purposes

## 📚 Related Resources

- [Criterion.rs Official Documentation](https://docs.rs/criterion/)
- [Rust Performance Analysis Guide](https://nnethercote.github.io/perf-book/)
- [mimalloc Research Paper](https://www.microsoft.com/en-us/research/uploads/prod/2019/06/mimalloc-tr-v1.pdf)
- [Memory Allocator Comparison Study](https://github.com/daanx/mimalloc-bench)

## 💡 Common Questions

### Q: Why is there a large performance difference between Debug and Release modes?

A: Auto-allocator uses system allocator in Debug mode (fast compilation) and selects high-performance allocators based on hardware in Release mode. This is by design, ensuring compilation speed during development and performance optimization in production.

### Q: How to interpret "Elements/sec" metrics?

A: This indicates the number of operations processed per second. For example, in allocation tests, it represents how many allocation operations can be completed per second. Higher values indicate better performance.

### Q: Why are concurrent test results unstable?

A: Concurrent performance is affected by system load, scheduling policies, etc. It's recommended to run multiple times in relatively idle environments and observe average performance trends.

### Q: How to compare performance between different versions?

A: Use Criterion's baseline functionality:
```bash
# Save current version
cargo bench --save-baseline v1.0
# Compare after version switch
cargo bench --baseline v1.0
```

### Q: Why doesn't auto-allocator use jemalloc anymore?

A: Based on extensive performance research, mimalloc has been shown to consistently outperform jemalloc in most scenarios, especially multi-threaded workloads. Auto-allocator now prioritizes mimalloc for modern platforms while falling back to system allocators for compatibility.