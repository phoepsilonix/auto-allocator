# Simple Demo Example

## 📖 Overview

This is the basic example for auto-allocator, demonstrating zero-configuration automatic allocator selection. Just add one import and get optimal memory performance automatically.

## 🚀 How to Run

```bash
# Debug mode - automatically selects system allocator (fast compilation)
cargo run --example simple_demo

# Release mode - automatically selects high-performance allocator
cargo run --release --example simple_demo
```

## 📊 Expected Output

### Debug Mode Output

```
🚀 Auto Allocator Demo
======================
✅ Selected allocator: System
📝 Reason: system allocator - debug build (16 cores, 128GB total RAM)

🎯 Runtime Hardware Selection:
- Debug builds: automatically use system allocator
- Release + modern platforms: automatically use mimalloc (best performance)
- WASM/Mobile platforms: system allocator for compatibility
- Embedded systems: specialized embedded allocator

🏆 Based on Microsoft and independent performance research:
  • mimalloc provides superior multi-threaded performance
  • Up to 1.6x faster than system allocators in complex workloads
  • Better memory efficiency and cross-platform support
💡 This is pure runtime selection - no configuration needed!

✅ Successfully allocated and populated Vec with 1000 bytes
✅ Successfully allocated String with 1300 bytes

📊 System Information:
  🖥️  OS: macos
  ⚙️  CPU Cores: 16
  🧠 Total Memory: 128GB
  🌐 WASM: false
  🐛 Debug Build: true
  🏗️  Architecture: aarch64
```

### Release Mode Output

```
🚀 Auto Allocator Demo
======================
✅ Selected allocator: Mimalloc
📝 Reason: mimalloc selected - optimal performance choice - runtime detected (16 cores, 128GB total RAM)

🎯 Runtime Hardware Selection:
- Debug builds: automatically use system allocator
- Release + modern platforms: automatically use mimalloc (best performance)
- WASM/Mobile platforms: system allocator for compatibility
- Embedded systems: specialized embedded allocator

🏆 Based on Microsoft and independent performance research:
  • mimalloc provides superior multi-threaded performance
  • Up to 1.6x faster than system allocators in complex workloads
  • Better memory efficiency and cross-platform support
💡 This is pure runtime selection - no configuration needed!

✅ Successfully allocated and populated Vec with 1000 bytes
✅ Successfully allocated String with 1300 bytes

📊 System Information:
  🖥️  OS: macos
  ⚙️  CPU Cores: 16
  🧠 Total Memory: 128GB
  🌐 WASM: false
  🐛 Debug Build: false
  🏗️  Architecture: aarch64
```

## 🧠 How It Works

Auto-allocator automatically selects the best allocator based on your environment:

| Environment | Selected Allocator | Reason |
|-------------|-------------------|---------|
| Debug builds | System | Fast compilation |
| Modern platforms (Release) | mimalloc | Superior performance |
| WASM/Mobile | System | Platform compatibility |
| Embedded systems | embedded-alloc | Resource optimization |

No configuration needed - just add `use auto_allocator;` to your code!

## 🔧 Common Issues

### WASM Testing

```bash
# WASM binaries cannot be executed by the host OS - they need a WASM runtime
cargo run --target wasm32-unknown-unknown --example simple_demo  # ❌ Will fail with "cannot execute binary file"

# Correct approaches:
cargo check --target wasm32-unknown-unknown --example simple_demo  # ✅ Compilation check
cargo build --target wasm32-unknown-unknown --example simple_demo  # ✅ Build binary
```

### Debugging Allocator Selection

```bash
# View detailed allocator selection process
cargo run --release --example simple_demo

# Compare debug vs release
cargo run --example simple_demo           # Debug mode
cargo run --release --example simple_demo # Release mode
```

