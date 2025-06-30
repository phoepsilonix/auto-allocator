/// Demonstrates runtime allocator optimization check functionality
#[allow(clippy::single_component_path_imports)]
use auto_allocator;

fn main() {
    // Initialize logging (skip on WASM)
    #[cfg(not(target_arch = "wasm32"))]
    env_logger::init();

    println!("=== Auto Allocator Optimization Check ===");
    println!();

    // Get current allocator information
    let info = auto_allocator::get_allocator_info();
    println!("Current Allocator:");
    println!("  Type: {:?}", info.allocator_type);
    println!("  Reason: {}", info.reason);
    println!();

    // Get recommendations for current runtime environment
    let (recommended_type, recommended_reason) = auto_allocator::get_recommended_allocator();
    println!("Recommended for Current Environment:");
    println!("  Type: {:?}", recommended_type);
    println!("  Reason: {}", recommended_reason);
    println!();

    // Check if current configuration is optimal
    let (is_optimal, suggestion) = auto_allocator::check_allocator_optimization();

    if is_optimal {
        println!("Current allocator is optimal for this environment!");
    } else {
        println!("Current allocator may not be optimal for this environment.");
        if let Some(msg) = suggestion {
            println!("Optimization suggestion:");
            println!("{}", msg);
        }
    }

    println!();
    println!("System Information:");
    println!("  OS: {}", info.system_info.os_type);
    println!("  CPU Cores: {}", info.system_info.cpu_cores);
    println!(
        "  Total Memory: {}",
        auto_allocator::format_memory_size(info.system_info.total_memory_bytes)
    );
    println!("  WASM: {}", info.system_info.is_wasm);
    println!("  Debug Build: {}", info.system_info.is_debug);
    println!("  Architecture: {}", info.system_info.target_arch);

    println!();
    println!("=== Performance Guidelines ===");
    println!();

    match recommended_type {
        auto_allocator::AllocatorType::Mimalloc => {
            println!("⚡ mimalloc is recommended for:");
            println!("   • High-performance applications and servers");
            println!("   • Multi-threaded applications");
            println!("   • Desktop applications and CLI tools");
            println!("   • Modern systems (Windows/macOS/Linux)");
            println!("   • Excellent cross-platform support and performance");
        }
        auto_allocator::AllocatorType::MimallocSecure => {
            println!("🔒 mimalloc-secure is recommended for:");
            println!("   • Security-critical applications");
            println!("   • Applications requiring protection against heap exploits");
            println!("   • High-performance with security hardening");
            println!("   • Note: ~10% performance overhead for security features");
        }
        auto_allocator::AllocatorType::System => {
            println!("🛡️ system allocator is recommended for:");
            println!("   • Debug builds and development");
            println!("   • Maximum compatibility requirements");
            println!("   • WASM applications");
            println!("   • Resource-constrained environments");
        }
        auto_allocator::AllocatorType::EmbeddedHeap => {
            println!("embedded allocator is recommended for:");
            println!("   • Embedded systems and microcontrollers");
            println!("   • No-std environments");
            println!("   • Memory-constrained applications");
            println!("   • Real-time systems requiring deterministic allocation");
        }
    }

    println!();
    println!("💡 Tip: auto-allocator uses pure runtime selection - no configuration needed!");
    println!("   Different modes automatically select optimal allocators:");
    println!("   cargo run --example optimization_check           # Debug → System allocator");
    println!("   cargo run --release --example optimization_check # Release → Performance-optimized allocator");
    println!("   The same binary automatically adapts to different hardware environments.");
}
