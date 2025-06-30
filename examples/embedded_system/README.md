# Auto-Allocator Embedded System Example

This example demonstrates auto-allocator's automatic platform detection and embedded-alloc selection in a bare-metal RISC-V environment. It shows how auto-allocator seamlessly adapts from standard library environments to embedded no_std systems.

## Why RISC-V?

RISC-V was chosen for this embedded example for several compelling reasons:

1. **Simplicity**: RISC-V has a clean, straightforward instruction set that's easier to work with than complex architectures like x86 or ARM
2. **Open Standard**: Unlike proprietary architectures, RISC-V is completely open, making it ideal for examples and education
3. **Excellent QEMU Support**: QEMU provides robust RISC-V emulation with virtual devices (UART, timers) that simplify bare-metal development
4. **Easy Output**: The QEMU RISC-V virt machine provides a simple UART at a known memory address, making it trivial to print debug output
5. **Growing Ecosystem**: RISC-V represents the future of open processor design, making it valuable to demonstrate

## Quick Start

### Prerequisites

1. **Rust toolchain** with RISC-V target support:
   ```bash
   rustup target add riscv64imac-unknown-none-elf
   ```

2. **QEMU** with RISC-V support:
   ```bash
   # macOS (with Homebrew)
   brew install qemu
   
   # Ubuntu/Debian
   sudo apt-get install qemu-system-riscv64
   
   # Arch Linux
   sudo pacman -S qemu-arch-extra
   ```

### Building and Running

1. **Standard environment** (shows different allocator):
   ```bash
   cargo run
   ```

2. **Embedded environment** (shows embedded-alloc):
   ```bash
   # Build for RISC-V embedded target
   cargo build --target riscv64imac-unknown-none-elf --release
   
   # Run in QEMU
   qemu-system-riscv64 \
     -machine virt \
     -cpu rv64 \
     -smp 1 \
     -m 128M \
     -serial stdio \
     -display none \
     -kernel target/riscv64imac-unknown-none-elf/release/embedded_system
   ```

## Expected Output

### Standard Environment
```
=== auto-allocator Embedded System Example ===

Current Environment Allocator:
  Type: System
  Reason: system allocator - debug build (16 cores, 128GB total RAM)

Expected allocator in embedded environment: EmbeddedHeap (embedded-alloc)
Actual allocator in this environment: System
```

### Embedded Environment (QEMU)
```
=== auto-allocator Embedded System Demo ===
Platform: RISC-V (riscv64imac-unknown-none-elf)
Allocator Type: EmbeddedHeap (embedded-alloc) [OK]
Selection Reason: embedded-alloc selected for no_std environment
Counter: 0 | Using embedded-alloc successfully!
Counter: 1 | Using embedded-alloc successfully!
Counter: 2 | Using embedded-alloc successfully!
Counter: 3 | Using embedded-alloc successfully!
Counter: 4 | Using embedded-alloc successfully!

=== auto-allocator Benefits Demonstrated ===
[OK] Automatic platform detection
[OK] Optimal allocator selection (embedded-alloc)
[OK] Zero-configuration setup
[OK] Runtime verification capability

Demo completed successfully!
```

## File Structure

```
embedded_system/
├── Cargo.toml          # Package configuration and dependencies
├── README.md           # This documentation
├── memory.x            # Linker script for memory layout
├── build.rs            # Build script for embedded-specific configuration
└── src/
    └── main.rs         # Main source code with dual std/no_std support
```

## Core Files Explained

### Cargo.toml
Defines the package configuration with:
- **Binary name**: `embedded_system` (matches directory)
- **Dependencies**: Links to the parent auto-allocator library
- **Build profiles**: Optimized settings for embedded development

### memory.x
Linker script that defines memory layout for RISC-V:
- **RAM origin**: `0x80200000` (avoids OpenSBI firmware at `0x80000000`)
- **RAM size**: 128KB (suitable for embedded systems)
- **Proper alignment**: Ensures correct memory placement

### build.rs
Build script that configures embedded-specific linking:
- **Conditional linking**: Only applies embedded linker script for `target_os = "none"`
- **Cross-platform compatibility**: Works on both macOS and Linux
- **Memory layout**: Links the `memory.x` file when building for embedded targets

### src/main.rs
The main source file with comprehensive conditional compilation:

#### Key Features:
1. **Dual compilation modes**:
   - Standard mode: Uses `std`, regular `main()` function
   - Embedded mode: Uses `no_std`, custom `_start()` entry point

2. **Platform detection verification**:
   - Calls `auto_allocator::get_allocator_info()` to verify correct allocator selection
   - Displays allocator type and selection reasoning
   - Shows clear success/error indicators

3. **Embedded system utilities**:
   - Direct UART register access for output
   - Custom `print_str()` and `print_num()` functions
   - Assembly-based delay loops

## Core Logic Explained

### Allocator Selection Process

1. **Compile time**: Auto-allocator's build script detects `target_os = "none"` and prints:
   ```
   warning: Auto-allocator: Embedded platform detected (riscv64)
   warning:   → Will use embedded-alloc for resource optimization
   ```

2. **Runtime**: The global allocator is automatically configured to use embedded-alloc

3. **Verification**: Code calls `get_allocator_info()` to confirm the selection:
   ```rust
   let allocator_info = auto_allocator::get_allocator_info();
   match allocator_info.allocator_type {
       auto_allocator::AllocatorType::EmbeddedHeap => {
           print_str(b"EmbeddedHeap (embedded-alloc) [OK]\n")
       },
       // ... handle other cases
   }
   ```

### Cross-Platform Compilation

The code uses conditional compilation to work in both environments:

```rust
// Standard library version
#[cfg(not(target_os = "none"))]
fn main() {
    println!("Running in std environment");
    // Use std::println! macro
}

// Embedded version
#[cfg(target_os = "none")]
pub extern "C" fn _start() -> ! {
    print_str(b"Running in no_std environment\n");
    // Use custom UART output
    loop {}
}
```

### Memory Safety

Even in embedded environments, the code maintains Rust's memory safety:
- **No unsafe allocations**: All memory management handled by embedded-alloc
- **Controlled unsafe blocks**: Only for hardware register access
- **Stack-based data**: Uses stack allocations for temporary data

## Troubleshooting

### Common Issues

1. **RISC-V target not installed**:
   ```bash
   rustup target add riscv64imac-unknown-none-elf
   ```

2. **QEMU not found**:
   Install QEMU with RISC-V support for your platform

3. **Linker errors**:
   Ensure `memory.x` is present and `build.rs` is properly configured

4. **No output in QEMU**:
   Verify the correct UART address (0x10000000) and use `-serial stdio`

### Platform Differences

- **macOS**: May require different QEMU installation methods
- **Linux**: Usually has QEMU packages in standard repositories
- **Windows**: Consider using WSL for easier QEMU setup

## Extension Ideas

This example can be extended to demonstrate:

1. **Real hardware**: Port to actual RISC-V development boards
2. **Heap allocation testing**: Add actual heap allocation examples
3. **Multiple allocators**: Compare performance between different allocators
4. **Custom allocators**: Implement domain-specific allocator strategies
5. **Memory profiling**: Add memory usage monitoring and reporting
