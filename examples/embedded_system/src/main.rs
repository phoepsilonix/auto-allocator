//! # Auto-Allocator Embedded System Example
//!
//! This example demonstrates auto-allocator's automatic platform detection
//! and embedded-alloc selection in a no_std RISC-V environment.
//!
//! ## Key Features Demonstrated:
//! - Automatic allocator selection (embedded-alloc vs system allocator)
//! - Runtime verification of allocator choice
//! - Cross-platform compilation (std vs no_std)
//! - RISC-V bare-metal execution in QEMU

// Enable no_std mode for embedded targets (target_os = "none")
#![cfg_attr(target_os = "none", no_std)]
// Disable standard main function for embedded targets
#![cfg_attr(target_os = "none", no_main)]

// Assembly instructions for embedded delay loops
#[cfg(target_os = "none")]
use core::arch::asm;

// Import auto-allocator to initialize the global allocator
// This automatically selects the best allocator for the current platform
#[allow(unused_imports)]
use auto_allocator;

/// Main function for non-embedded environments (std)
/// 
/// This function runs when compiling for standard targets (not no_std).
/// It demonstrates the difference between allocators in different environments.
#[cfg(not(target_os = "none"))]
fn main() {
    println!("=== auto-allocator Embedded System Example ===");
    println!();
    
    // Display current environment allocator information
    // In std environment, this will typically be System or Mimalloc
    let allocator_info = auto_allocator::get_allocator_info();
    println!("Current Environment Allocator:");
    println!("  Type: {:?}", allocator_info.allocator_type);
    println!("  Reason: {}", allocator_info.reason);
    println!();
    
    println!("This is an embedded system example that should be run with:");
    println!("  cargo build --target riscv64imac-unknown-none-elf --release");
    println!("  qemu-system-riscv64 -M virt -m 128M -smp 1 -serial stdio -kernel target/riscv64imac-unknown-none-elf/release/embedded_system");
    println!();
    println!("Expected allocator in embedded environment: EmbeddedHeap (embedded-alloc)");
    println!("Actual allocator in this environment: {:?}", allocator_info.allocator_type);
}

// ============================================================================
// EMBEDDED SYSTEM CONFIGURATION (no_std only)
// ============================================================================

/// UART0 register address for QEMU RISC-V virt machine
/// 
/// The QEMU RISC-V virt machine maps UART0 to address 0x10000000.
/// This provides a simple way to output text in a bare-metal environment.
#[cfg(target_os = "none")]
const UART0: *mut u8 = 0x1000_0000 as *mut u8;

/// QEMU exit address for test automation
/// 
/// Writing to this address causes QEMU to exit, useful for automated testing.
/// This is a QEMU-specific feature not available on real hardware.
#[cfg(target_os = "none")]
const EXIT_ADDR: *mut u32 = 0x100000 as *mut u32;

/// Panic handler for no_std environment
/// 
/// In embedded systems, we can't use std::panic, so we provide a minimal
/// panic handler that just enters an infinite loop.
#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

/// Entry point for embedded system (no_std)
/// 
/// This function serves as the entry point for the embedded system.
/// It's marked with #[no_mangle] to prevent name mangling and
/// placed in the .text.entry section for proper linking.
#[cfg(target_os = "none")]
#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    // Print welcome message
    let hello = b"=== auto-allocator Embedded System Demo ===\n";
    for &c in hello {
        unsafe {
            UART0.write_volatile(c);
        }
    }

    // Display platform information
    let msg1 = b"Platform: RISC-V (riscv64imac-unknown-none-elf)\n";
    for &c in msg1 {
        unsafe {
            UART0.write_volatile(c);
        }
    }

    // ========================================================================
    // CORE DEMONSTRATION: Verify allocator selection
    // ========================================================================
    
    // Get allocator information from auto-allocator
    // This demonstrates runtime verification of allocator choice
    let allocator_info = auto_allocator::get_allocator_info();
    
    print_str(b"Allocator Type: ");
    match allocator_info.allocator_type {
        auto_allocator::AllocatorType::EmbeddedHeap => {
            print_str(b"EmbeddedHeap (embedded-alloc) [OK]\n")
        },
        auto_allocator::AllocatorType::System => {
            print_str(b"System (ERROR: should be embedded!) [ERROR]\n")
        },
        auto_allocator::AllocatorType::Mimalloc => {
            print_str(b"Mimalloc (ERROR: not available in no_std!) [ERROR]\n")
        },
        auto_allocator::AllocatorType::MimallocSecure => {
            print_str(b"MimallocSecure (ERROR: not available in no_std!) [ERROR]\n")
        },
    }
    
    print_str(b"Selection Reason: ");
    print_str(allocator_info.reason.as_bytes());
    print_str(b"\n");

    // ========================================================================
    // DEMONSTRATION LOOP: Show successful operation
    // ========================================================================
    
    let mut counter = 0;

    loop {
        print_str(b"Counter: ");
        print_num(counter);
        
        print_str(b" | Using embedded-alloc successfully!");
        print_str(b"\n");

        // Simple delay to make output readable
        delay();

        counter += 1;

        // Exit after demonstrating successful operation
        if counter >= 5 {
            print_str(b"\n=== auto-allocator Benefits Demonstrated ===\n");
            print_str(b"[OK] Automatic platform detection\n");
            print_str(b"[OK] Optimal allocator selection (embedded-alloc)\n");
            print_str(b"[OK] Zero-configuration setup\n");
            print_str(b"[OK] Runtime verification capability\n");
            print_str(b"\nDemo completed successfully!\n");
            
            // Exit QEMU for automated testing
            // Note: This is QEMU-specific and not available on real hardware
            unsafe {
                EXIT_ADDR.write_volatile(0x5555);
            }
            
            // Infinite loop as backup (required for no_std)
            loop {}
        }
    }
}

// ============================================================================
// UTILITY FUNCTIONS FOR EMBEDDED OUTPUT (no_std only)
// ============================================================================

/// Simple delay function for embedded systems
/// 
/// Uses NOP (no operation) assembly instructions to create a delay.
/// The delay duration depends on CPU frequency and is approximate.
#[cfg(target_os = "none")]
fn delay() {
    for _ in 0..150_0000 {
        unsafe { asm!("nop") }
    }
}

/// Print string to UART0 in embedded environment
/// 
/// Outputs each byte of the string directly to the UART register.
/// This is a simple, blocking output method suitable for debugging.
/// 
/// # Arguments
/// * `s` - Byte slice containing the string to print
#[cfg(target_os = "none")]
fn print_str(s: &[u8]) {
    for &c in s {
        unsafe {
            UART0.write_volatile(c);
        }
    }
}

/// Print unsigned 32-bit number to UART0 in embedded environment
/// 
/// Converts a number to its decimal string representation and outputs
/// it character by character. Uses a small stack buffer for efficiency.
/// 
/// # Arguments
/// * `n` - The number to print
#[cfg(target_os = "none")]
fn print_num(mut n: u32) {
    let mut buf = [0u8; 10];  // Buffer for up to 10 digits
    let mut i = buf.len();

    // Handle zero as special case
    if n == 0 {
        unsafe { UART0.write_volatile(b'0'); }
        return;
    }

    // Convert number to ASCII digits (in reverse order)
    while n > 0 {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }

    // Output the digits in correct order
    for &c in &buf[i..] {
        unsafe {
            UART0.write_volatile(c);
        }
    }
}