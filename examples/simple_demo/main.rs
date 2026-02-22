/// 🚀 Auto-Allocator Basic Usage Demo
///
/// This example demonstrates the core functionality of auto-allocator:
/// 1. 🎯 Zero-configuration automatic allocator selection
/// 2. 📊 System information viewing
/// 3. ⚙️ Environment variable control methods
/// 4. 🧪 Basic memory allocation testing

// This is the core usage of auto-allocator: just one use statement enables automatic allocator selection
#[allow(clippy::single_component_path_imports)]
use auto_allocator;

fn main() {
    // Initialize logging (skip on WASM)
    #[cfg(not(target_arch = "wasm32"))]
    env_logger::init();

    println!("🚀 Auto Allocator Demo");
    println!("======================");

    // Get allocator information
    let info = auto_allocator::get_allocator_info();

    println!("✅ Selected allocator: {:?}", info.allocator_type);
    println!("📝 Reason: {}", info.reason);
    println!();

    // Demonstrate automatic selection features
    println!("🎯 Runtime Hardware Selection:");
    println!("- Debug builds: automatically use system allocator");
    println!("- Release + modern platforms: automatically use mimalloc (best performance)");
    println!("- WASM/Mobile platforms: system allocator for compatibility");
    println!("- Embedded systems: specialized embedded allocator");
    println!();
    
    println!("🏆 Based on Microsoft and independent performance research:");
    println!("  • mimalloc provides superior multi-threaded performance");
    println!("  • Up to 1.6x faster than system allocators in complex workloads");
    println!("  • Better memory efficiency and cross-platform support");
    println!("💡 This is pure runtime selection - no configuration needed!");
    println!();

    // Demonstrate basic memory allocation
    let data: Vec<u8> = (0..1000).map(|i| (i % 256) as u8).collect();
    println!(
        "✅ Successfully allocated and populated Vec with {} bytes",
        data.len()
    );

    let text = "Hello, world!".repeat(100);
    println!("✅ Successfully allocated String with {} bytes", text.len());

    println!();
    println!("📊 System Information:");
    println!("  🖥️  OS: {}", info.system_info.os_type);
    println!("  ⚙️  CPU Cores: {}", info.system_info.cpu_cores);
    println!(
        "  🧠 Total Memory: {}",
        auto_allocator::format_memory_size(info.system_info.total_memory_bytes)
    );
    println!("  🌐 WASM: {}", info.system_info.is_wasm);
    println!("  🐛 Debug Build: {}", info.system_info.is_debug);
    println!("  🏗️  Architecture: {}", info.system_info.target_arch);
}
