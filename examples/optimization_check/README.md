# Optimization Check Example

## 📖 Overview

This example demonstrates how to use auto-allocator's runtime optimization check functionality to verify whether the current allocator is optimal for your environment.

## 🎯 Use Cases

- **Performance Tuning** - Check if the current allocator is suitable for production environments
- **Cross-platform Deployment** - Verify allocator selection across different platforms
- **Environment Analysis** - Understand system hardware specifications and recommended allocator types
- **Development Workflow** - Compare debug vs release mode allocator choices

## 🚀 How to Run

### Basic Check
```bash
# Check current environment's allocator optimization status
cargo run --example optimization_check

# Check in Release mode (usually more meaningful)
cargo run --release --example optimization_check
```


## 📊 Expected Output

### Debug Mode Example
```
=== Auto Allocator Optimization Check ===

Current Allocator:
  Type: System
  Reason: system allocator - debug build (16 cores, 128GB total RAM)

Recommended for Current Environment:
  Type: System
  Reason: system allocator - debug build (16 cores, 128GB total RAM)

Current allocator is optimal for this environment!

System Information:
  OS: macos
  CPU Cores: 16
  Total Memory: 128GB
  WASM: false
  Debug Build: true
  Architecture: aarch64

=== Performance Guidelines ===

🛡️ system allocator is recommended for:
   • Debug builds and development
   • Maximum compatibility requirements
   • WASM applications
   • Resource-constrained environments

💡 Tip: auto-allocator uses pure runtime selection - no configuration needed!
   Different modes automatically select optimal allocators:
   cargo run --example optimization_check           # Debug → System allocator
   cargo run --release --example optimization_check # Release → Performance-optimized allocator
   The same binary automatically adapts to different hardware environments.
```

### Release Mode Example (Modern Platform)
```
=== Auto Allocator Optimization Check ===

Current Allocator:
  Type: Mimalloc
  Reason: mimalloc selected - optimal performance choice - runtime detected (16 cores, 128GB total RAM)

Recommended for Current Environment:
  Type: Mimalloc
  Reason: mimalloc selected - optimal performance choice - runtime detected (16 cores, 128GB total RAM)

Current allocator is optimal for this environment!

System Information:
  OS: macos
  CPU Cores: 16
  Total Memory: 128GB
  WASM: false
  Debug Build: false
  Architecture: aarch64

=== Performance Guidelines ===

⚡ mimalloc is recommended for:
   • High-performance applications and servers
   • Multi-threaded applications
   • Desktop applications and CLI tools
   • Modern systems (Windows/macOS/Linux)
   • Excellent cross-platform support and performance

💡 Tip: auto-allocator uses pure runtime selection - no configuration needed!
   Different modes automatically select optimal allocators:
   cargo run --example optimization_check           # Debug → System allocator
   cargo run --release --example optimization_check # Release → Performance-optimized allocator
   The same binary automatically adapts to different hardware environments.
```

## 🔍 Code Analysis

This example demonstrates the optimization check APIs:

1. **Current allocator info**: `auto_allocator::get_allocator_info()`
2. **Recommended allocator**: `auto_allocator::get_recommended_allocator()`
3. **Optimization check**: `auto_allocator::check_allocator_optimization()`

The optimization check compares the current allocator with the recommended one and provides suggestions if they differ.

## 🔧 Common Issues

### Different Results in Debug vs Release

This is expected behavior:
- **Debug mode**: Always uses system allocator for fast compilation
- **Release mode**: Uses performance-optimized allocator based on platform

### Platform-Specific Behavior

- **Modern platforms** (Windows/macOS/Linux): mimalloc in release mode
- **Mobile platforms** (Android/iOS): system allocator for platform compliance
- **WASM**: system allocator for browser compatibility
- **Embedded**: embedded-alloc for resource constraints
