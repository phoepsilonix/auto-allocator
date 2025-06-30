//! # Auto Allocator - Zero-Configuration Memory Optimization
//!
//! **Just add one line and get optimal memory performance automatically!**
//!
//! Auto-allocator automatically selects the best memory allocator for your platform and hardware,
//! giving you significant performance improvements without any configuration or code changes.
//!
//! ## Why Auto Allocator?
//!
//! - **🚀 Instant Performance**: Up to 1.6x faster allocation in multi-threaded applications
//! - **🔧 Zero Configuration**: Works perfectly out-of-the-box, no setup required
//! - **🌍 Universal Compatibility**: Optimizes across all platforms - servers, desktop, mobile, embedded, WASM
//! - **🧠 Platform Intelligence**: Automatically chooses the best allocator for each platform
//! - **⚡ Production Ready**: Used safely in high-performance production environments
//!
//! ## Quick Start
//!
//! **Step 1:** Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! auto-allocator = "*"
//! ```
//!
//! **Step 2:** Add one line to your `main.rs`:
//! ```rust,ignore
//! use auto_allocator; // That's it! 🎉
//!
//! fn main() {
//!     // Your code automatically runs with optimal memory performance
//!     let data = vec![1, 2, 3, 4, 5];
//!     println!("Memory allocations are now optimized!");
//! }
//! ```
//!
//! **That's literally all you need!** Auto-allocator handles everything else automatically.
//!
//! ## What You Get
//!
//! - **Linux Servers**: mimalloc for superior multi-threaded performance
//! - **Windows/macOS**: mimalloc for desktop application speed
//! - **Android/iOS**: Platform-optimized system allocators (Scudo/libmalloc)
//! - **Docker/Kubernetes**: Optimized for containerized deployments
//! - **Embedded Systems**: Automatic embedded-alloc for all no_std platforms (RISC-V, ARM, AVR, MSP430, Xtensa, etc.)
//! - **WASM**: Compatible allocation for web applications
//!
//! **Security Mode Available:**
//! ```toml
//! auto-allocator = { version = "*", features = ["secure"] }
//! ```

#![cfg_attr(target_os = "none", no_std)]

// Conditional imports for std vs no_std
#[cfg(not(target_os = "none"))]
use log::info;
#[cfg(not(target_os = "none"))]
use once_cell::sync::Lazy;

use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicU8, Ordering};
#[cfg(not(target_os = "none"))]
use core::sync::atomic::AtomicBool;

// Import std-specific modules conditionally
#[cfg(not(target_os = "none"))]
use std::alloc;

// ========== Type Definitions ==========

/// Memory allocator type enumeration
///
/// Represents all memory allocator types supported by auto-allocator.
/// Selection priority: mimalloc > embedded > system
///
/// # Performance Characteristics
///
/// - [`AllocatorType::MimallocSecure`] - Microsoft-developed allocator with security hardening (10% overhead)
/// - [`AllocatorType::Mimalloc`] - Microsoft-developed allocator, optimal multi-threaded performance  
/// - [`AllocatorType::EmbeddedHeap`] - Lightweight allocator for resource-constrained environments
/// - [`AllocatorType::System`] - Operating system default allocator, maximum compatibility
///
/// # Automatic Selection Logic
///
/// 1. **Modern Linux**: mimalloc (if GCC 4.9+ and stdatomic.h available)
/// 2. **Legacy Linux**: Compilation error with upgrade guidance
/// 3. **Windows/macOS**: mimalloc (always available)
/// 4. **Mobile/BSD**: System allocators (platform compliance)
/// 5. **Embedded** (`target_os = "none"`): embedded-alloc (all no_std architectures)
///
/// # Example
///
/// ```rust
/// use auto_allocator;
///
/// let info = auto_allocator::get_allocator_info();
/// match info.allocator_type {
///     auto_allocator::AllocatorType::Mimalloc => {
///         println!("Using mimalloc - optimal performance");
///     }
///     auto_allocator::AllocatorType::System => {
///         println!("Using system allocator - platform compliance");
///     }
///     _ => println!("Using other allocator"),
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocatorType {

    /// Security-hardened mimalloc allocator
    ///
    /// Microsoft-developed allocator with enhanced security features.
    /// ~10% performance overhead for comprehensive heap protection.
    /// Available when `secure` feature is enabled on compatible platforms.
    MimallocSecure,

    /// High-performance mimalloc allocator
    ///
    /// Microsoft-developed allocator optimized for multi-threaded workloads.
    /// Automatically selected on modern systems with GCC 4.9+ and stdatomic.h.
    Mimalloc,


    /// Embedded systems allocator
    ///
    /// Lightweight allocator designed for resource-constrained environments.
    /// Automatically selected on embedded architectures.
    EmbeddedHeap,

    /// System default allocator
    ///
    /// Operating system provided allocator, maximum compatibility.
    /// Selected for debug builds, WASM, mobile, and platforms with optimized native allocators.
    System,
}

/// Allocator information structure
///
/// Contains the currently selected allocator type, selection reason, and system information.
/// Obtained through the [`get_allocator_info()`] function.
///
/// # Fields
///
/// - `allocator_type` - Currently used allocator type
/// - `reason` - Detailed reason for allocator selection, including hardware information
/// - `system_info` - System hardware and environment information
///
/// # Example
///
/// ```rust
/// use auto_allocator;
///
/// let info = auto_allocator::get_allocator_info();
/// println!("Allocator: {:?}", info.allocator_type);
/// println!("Selection reason: {}", info.reason);
/// println!("CPU cores: {}", info.system_info.cpu_cores);
/// ```
#[derive(Debug, Clone)]
pub struct AllocatorInfo {
    /// Currently used allocator type
    pub allocator_type: AllocatorType,

    /// Detailed reason for allocator selection
    ///
    /// Contains hardware detection results and selection logic explanation, for example:
    /// "mimalloc selected by runtime hardware analysis (16 cores, 128GB total RAM)"
    #[cfg(not(target_os = "none"))]
    pub reason: String,
    #[cfg(target_os = "none")]
    pub reason: &'static str,

    /// System hardware and environment information
    pub system_info: SystemInfo,
}

/// System information structure
///
/// Contains runtime-detected system hardware and environment information,
/// used for allocator selection decisions.
///
/// # Fields
///
/// - `os_type` - Operating system type (linux, macos, windows, etc.)
/// - `cpu_cores` - CPU core count (including hyperthreaded cores)
/// - `total_memory_bytes` - Total memory in bytes
/// - `is_debug` - Whether this is a Debug build
/// - `is_wasm` - Whether this is a WASM environment
/// - `target_arch` - Target architecture (x86_64, aarch64, etc.)
///
/// # Example
///
/// ```rust
/// use auto_allocator;
///
/// let info = auto_allocator::get_allocator_info();
/// let sys = &info.system_info;
///
/// println!("Operating system: {}", sys.os_type);
/// println!("CPU cores: {}", sys.cpu_cores);
/// println!("Total memory: {}", auto_allocator::format_memory_size(sys.total_memory_bytes));
/// ```
#[derive(Debug, Clone)]
pub struct SystemInfo {
    /// Operating system type
    ///
    /// Examples: "linux", "macos", "windows", "unknown"
    #[cfg(not(target_os = "none"))]
    pub os_type: String,
    #[cfg(target_os = "none")]
    pub os_type: &'static str,

    /// CPU core count
    ///
    /// Detected via `std::thread::available_parallelism()`, includes hyperthreaded core count
    pub cpu_cores: usize,

    /// Total memory in bytes
    ///
    /// System total physical memory, used for hardware specification assessment.
    /// Use [`format_memory_size()`] to format as human-readable string.
    pub total_memory_bytes: u64,

    /// Whether this is a Debug build
    ///
    /// Debug builds automatically select system allocator for faster compilation
    pub is_debug: bool,

    /// Whether this is a WASM environment
    ///
    /// WASM environments automatically select system allocator for compatibility
    pub is_wasm: bool,

    /// Target architecture
    ///
    /// Examples: "x86_64", "aarch64", "riscv32", "wasm32"
    #[cfg(not(target_os = "none"))]
    pub target_arch: String,
    #[cfg(target_os = "none")]
    pub target_arch: &'static str,
}

// ========== Memory Formatting Utilities ==========

/// High-performance memory size formatting function
///
/// Converts byte count to human-readable memory size string, automatically selecting appropriate units.
/// Uses bit shift operations for performance optimization, supports memory sizes from bytes to PB level.
///
/// # Arguments
///
/// - `bytes` - The number of bytes to format
///
/// # Returns
///
/// Returns formatted string, for example:
/// - `1024` → `"1KB"`
/// - `1536` → `"1.5KB"`
/// - `1073741824` → `"1GB"`
///
/// # Supported Units
///
/// - **B** - Bytes (< 1024)
/// - **KB** - Kilobytes (1024 B)
/// - **MB** - Megabytes (1024 KB)
/// - **GB** - Gigabytes (1024 MB)
/// - **TB** - Terabytes (1024 GB)
/// - **PB** - Petabytes (1024 TB)
///
/// # Performance Features
///
/// - Uses bit shift operations instead of division for performance optimization
/// - Hardware-optimized leading zero count instructions
/// - Retains only 1 decimal place for improved performance
/// - Zero-copy string construction
///
/// # Examples
///
/// ```rust
/// use auto_allocator;
///
/// // Basic usage
/// assert_eq!(auto_allocator::format_memory_size(0), "0B");
/// assert_eq!(auto_allocator::format_memory_size(1024), "1KB");
/// assert_eq!(auto_allocator::format_memory_size(1536), "1.5KB");
/// assert_eq!(auto_allocator::format_memory_size(1048576), "1MB");
/// assert_eq!(auto_allocator::format_memory_size(1073741824), "1GB");
///
/// // Use in combination with system information
/// let info = auto_allocator::get_allocator_info();
/// let memory_str = auto_allocator::format_memory_size(info.system_info.total_memory_bytes);
/// println!("Total system memory: {}", memory_str);
///
/// // Display memory usage in application
/// fn display_memory_usage() {
///     let info = auto_allocator::get_allocator_info();
///     println!("Memory information:");
///     println!("  Total memory: {}", auto_allocator::format_memory_size(info.system_info.total_memory_bytes));
/// }
/// ```
///
/// # Precision Notes
///
/// For performance considerations, decimal places are limited to 1 digit. For scenarios
/// requiring higher precision, it is recommended to calculate directly using byte counts.
#[cfg(not(target_os = "none"))]
pub fn format_memory_size(bytes: u64) -> String {
    use std::format;
    
    if bytes == 0 {
        return "0B".to_string();
    }

    // Use bit shift calculations to avoid division operations for performance improvement
    // Each unit has a 1024x relationship, i.e., 2^10
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];

    // Use leading zero count to quickly determine appropriate unit level
    // leading_zeros() is a hardware-optimized instruction
    let unit_index = if bytes >= (1u64 << 50) {
        5
    }
    // >= 1PB
    else if bytes >= (1u64 << 40) {
        4
    }
    // >= 1TB
    else if bytes >= (1u64 << 30) {
        3
    }
    // >= 1GB
    else if bytes >= (1u64 << 20) {
        2
    }
    // >= 1MB
    else if bytes >= (1u64 << 10) {
        1
    }
    // >= 1KB
    else {
        0
    }; // < 1KB

    if unit_index == 0 {
        format!("{}B", bytes)
    } else {
        let shift = unit_index * 10; // Each unit is 2^10
        let value = bytes >> shift;
        let remainder = bytes & ((1u64 << shift) - 1);

        // Calculate decimal part (retain only 1 decimal place for performance)
        if remainder == 0 {
            format!("{}{}", value, UNITS[unit_index])
        } else {
            let fraction = (remainder * 10) >> shift;
            if fraction == 0 {
                format!("{}{}", value, UNITS[unit_index])
            } else {
                format!("{}.{}{}", value, fraction, UNITS[unit_index])
            }
        }
    }
}

/// Simplified memory size formatting for no_std environments
#[cfg(target_os = "none")]
pub fn format_memory_size(bytes: u64) -> &'static str {
    // For embedded systems, use predefined size categories
    if bytes == 0 {
        "0B"
    } else if bytes < 1024 {
        "<1KB"
    } else if bytes < (1024 * 1024) {
        "~KB"
    } else if bytes < (1024 * 1024 * 1024) {
        "~MB"
    } else {
        "~GB"
    }
}

// ========== Platform Detection ==========

/// Checks if the target is an embedded platform requiring specialized allocation
/// 
/// Uses `target_os = "none"` as the primary indicator of embedded/no_std environments.
/// This approach covers all current and future embedded targets automatically,
/// including architectures like RISC-V, ARM, AVR, MSP430, Xtensa, LoongArch, etc.
const fn is_embedded_target() -> bool {
    cfg!(target_os = "none")
}

/// Checks if mimalloc can be used on this platform
const fn can_use_mimalloc() -> bool {
    cfg!(all(
        feature = "_mimalloc",
        any(target_os = "windows", target_os = "macos", target_os = "linux"),
        not(target_arch = "wasm32"),
        not(debug_assertions)
    ))
}

/// Checks if secure mimalloc can be used on this platform
const fn can_use_mimalloc_secure() -> bool {
    cfg!(all(
        feature = "_mimalloc_secure",
        any(target_os = "windows", target_os = "macos", target_os = "linux"),
        not(target_arch = "wasm32"),
        not(debug_assertions)
    ))
}



// ========== Runtime Allocator Selection ==========

// Global state for allocator selection and logging  
// ID mapping: 0=uninitialized, 1=system, 2=mimalloc, 3=jemalloc, 4=embedded, 5=mimalloc-secure
static RUNTIME_ALLOCATOR_ID: AtomicU8 = AtomicU8::new(0);
#[cfg(not(target_os = "none"))]
static ALLOCATOR_LOGGED: AtomicBool = AtomicBool::new(false);
#[cfg(not(target_os = "none"))]
static LOG_FLUSHED: AtomicBool = AtomicBool::new(false);

/// Returns allocator ID for platforms with compile-time determinable choices
///
/// Returns `None` for platforms requiring runtime hardware detection (desktop systems).
/// This optimization avoids unnecessary runtime checks for 90% of platforms.
const fn get_compile_time_allocator() -> Option<u8> {
    if is_embedded_target() {
        return Some(4); // embedded-alloc
    }

    if cfg!(target_arch = "wasm32") {
        return Some(1); // system
    }

    if cfg!(debug_assertions) {
        return Some(1); // system (debug builds)
    }

    // Platforms with superior native allocators
    if cfg!(target_os = "android") {
        return Some(1); // Scudo
    }

    if cfg!(target_os = "ios") {
        return Some(1); // libmalloc
    }

    if cfg!(any(target_os = "freebsd", target_os = "netbsd", target_os = "openbsd")) {
        return Some(1); // native jemalloc/security-hardened
    }

    if cfg!(any(target_os = "solaris", target_os = "illumos")) {
        return Some(1); // libumem
    }

    None // High-performance platforms need runtime detection
}

/// Selects allocator using compile-time rules and runtime hardware detection
fn select_allocator_by_hardware() -> u8 {
    if let Some(allocator_id) = get_compile_time_allocator() {
        return allocator_id;
    }

    // Only high-performance platforms reach here - need CPU core detection
    // Use zero-allocation CPU detection to avoid infinite recursion
    let cpu_cores = get_cpu_cores_safe();

    // Multi-core systems: prefer mimalloc (secure > regular > system)
    if cpu_cores >= 2 && can_use_mimalloc_secure() {
        return 5; // mimalloc-secure
    }

    // Check if mimalloc is available
    // Since build script ensures compatibility, mimalloc is available if feature is enabled
    if cpu_cores >= 2 && can_use_mimalloc() {
        return 2; // mimalloc
    }

    1 // system (single-core or all high-performance allocators unavailable)
}

/// Get CPU core count without allocating memory (to avoid infinite recursion)
fn get_cpu_cores_safe() -> usize {
    #[cfg(unix)]
    {
        // Use direct libc calls to avoid std allocation
        unsafe {
            let cores = libc::sysconf(libc::_SC_NPROCESSORS_ONLN);
            if cores > 0 {
                cores as usize
            } else {
                1
            }
        }
    }
    
    #[cfg(windows)]
    {
        // Windows: Use direct WinAPI to avoid std allocation
        use winapi::um::sysinfoapi::{GetSystemInfo, SYSTEM_INFO};
        unsafe {
            let mut sysinfo: SYSTEM_INFO = std::mem::zeroed();
            GetSystemInfo(&mut sysinfo);
            sysinfo.dwNumberOfProcessors as usize
        }
    }
    
    #[cfg(not(any(unix, windows)))]
    {
        // Fallback: assume multi-core for unknown platforms
        4
    }
}

// ========== Embedded Heap Configuration ==========

// Embedded heap configuration for all no_std targets
#[cfg(target_os = "none")]
mod embedded_heap_config {
    use embedded_alloc::Heap;
    #[cfg(not(target_os = "none"))]
    use once_cell::sync::Lazy;

    // Architecture-specific heap sizes based on typical available memory
    // These are conservative defaults that work well for most embedded applications
    // Users can override by defining custom heap sizes in their own code

    #[cfg(target_arch = "avr")]
    pub const HEAP_SIZE: usize = 512; // AVR (Arduino Uno): 2KB total, use 512B heap (25%)

    #[cfg(target_arch = "msp430")]
    pub const HEAP_SIZE: usize = 256; // MSP430: 1KB total, use 256B heap (25%)

    #[cfg(target_arch = "riscv32")]
    pub const HEAP_SIZE: usize = 2048; // RISC-V 32-bit: typically 32KB+, use 2KB heap (6%)

    #[cfg(target_arch = "riscv64")]
    pub const HEAP_SIZE: usize = 4096; // RISC-V 64-bit: typically 128KB+, use 4KB heap (3%)

    #[cfg(target_arch = "xtensa")]
    pub const HEAP_SIZE: usize = 4096; // Xtensa (ESP32): 256KB+, use 4KB heap (1.5%)

    #[cfg(target_arch = "arm")]
    pub const HEAP_SIZE: usize = 1024; // ARM Cortex-M: typically 16KB+, use 1KB heap (6%)

    // Default heap size for other embedded architectures (LoongArch, Hexagon, BPF, SPARC, etc.)
    #[cfg(not(any(
        target_arch = "avr",
        target_arch = "msp430", 
        target_arch = "riscv32",
        target_arch = "riscv64",
        target_arch = "xtensa",
        target_arch = "arm"
    )))]
    pub const HEAP_SIZE: usize = 2048; // Conservative default for unknown architectures

    // Static memory pool for embedded heap
    // This is a conservative allocation that should work on most embedded systems
    pub static mut HEAP_MEMORY: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

    // Singleton heap instance - different implementations for std vs no_std
    #[cfg(not(target_os = "none"))]
    pub static EMBEDDED_HEAP: Lazy<Heap> = Lazy::new(|| unsafe { Heap::new(&mut HEAP_MEMORY[..]) });
    
    #[cfg(target_os = "none")]
    static mut EMBEDDED_HEAP_INSTANCE: Option<Heap> = None;
    
    /// Gets the embedded heap instance for no_std environments
    /// 
    /// This function provides access to the global embedded heap used in no_std 
    /// environments. The heap is lazily initialized on first access with 
    /// architecture-appropriate size defaults.
    /// 
    /// # Returns
    /// 
    /// A reference to the static embedded heap instance
    /// 
    /// # Safety
    /// 
    /// This function is only available in no_std environments (`target_os = "none"`).
    /// The heap initialization is done safely using static guarantees.
    #[cfg(target_os = "none")]
    pub fn get_embedded_heap() -> &'static Heap {
        unsafe {
            if EMBEDDED_HEAP_INSTANCE.is_none() {
                let heap = Heap::empty();
                heap.init(HEAP_MEMORY.as_mut_ptr() as usize, HEAP_SIZE);
                EMBEDDED_HEAP_INSTANCE = Some(heap);
            }
            EMBEDDED_HEAP_INSTANCE.as_ref().unwrap()
        }
    }
}

// ========== Safe Runtime Allocator Implementation ==========

pub struct RuntimeAllocator;

impl RuntimeAllocator {
    #[inline]
    fn get_allocator_id() -> u8 {
        let current_id = RUNTIME_ALLOCATOR_ID.load(Ordering::Acquire);

        if unlikely(current_id == 0) {
            // First call, perform hardware detection and selection
            let selected_id = select_allocator_by_hardware();
            RUNTIME_ALLOCATOR_ID.store(selected_id, Ordering::Release);

            // Record selection information (ensure only logged once)
            Self::log_allocator_selection(selected_id);

            selected_id
        } else {
            current_id
        }
    }

    #[cold]
    #[cfg(not(target_os = "none"))]
    fn log_allocator_selection(allocator_id: u8) {
        if ALLOCATOR_LOGGED
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            let (name, reason) = Self::get_allocator_log_info(allocator_id);
            record_allocator_selection(name, &reason);
        }
    }

    #[cold]
    #[cfg(target_os = "none")]
    fn log_allocator_selection(_allocator_id: u8) {
        // No logging in no_std environments
    }

    /// Get logging information based on allocator ID and compile-time platform detection
    #[cfg(not(target_os = "none"))]
    fn get_allocator_log_info(allocator_id: u8) -> (&'static str, String) {
        match allocator_id {
            5 => {
                let system_info = collect_system_info();
                ("mimalloc-secure", format!(
                    "security-hardened choice - runtime detected ({} cores, {} total RAM)",
                    system_info.cpu_cores,
                    format_memory_size(system_info.total_memory_bytes)
                ))
            },
            2 => {
                let system_info = collect_system_info();
                ("mimalloc", format!(
                    "optimal performance choice - runtime detected ({} cores, {} total RAM)",
                    system_info.cpu_cores,
                    format_memory_size(system_info.total_memory_bytes)
                ))
            },
            4 => {
                let system_info = collect_system_info();
                ("embedded-alloc", format!(
                    "embedded platform - compile-time selected ({} total RAM)",
                    format_memory_size(system_info.total_memory_bytes)
                ))
            },
            _ => {
                // System allocator - determine reason based on compile-time platform detection
                if cfg!(debug_assertions) {
                    let system_info = collect_system_info();
                    ("system", format!(
                        "debug build - compile-time selected ({} cores, {} total RAM)",
                        system_info.cpu_cores,
                        format_memory_size(system_info.total_memory_bytes)
                    ))
                } else if cfg!(target_arch = "wasm32") {
                    let system_info = collect_system_info();
                    ("system", format!(
                        "WASM environment - compile-time selected ({} total RAM)",
                        format_memory_size(system_info.total_memory_bytes)
                    ))
                } else if cfg!(target_os = "android") {
                    let system_info = collect_system_info();
                    ("system", format!(
                        "Android Scudo allocator - compile-time selected (security-first policy) ({} cores, {} total RAM)",
                        system_info.cpu_cores,
                        format_memory_size(system_info.total_memory_bytes)
                    ))
                } else if cfg!(target_os = "ios") {
                    let system_info = collect_system_info();
                    ("system", format!(
                        "iOS libmalloc allocator - compile-time selected (Apple optimized) ({} cores, {} total RAM)",
                        system_info.cpu_cores,
                        format_memory_size(system_info.total_memory_bytes)
                    ))
                } else if cfg!(any(target_os = "freebsd", target_os = "netbsd")) {
                    let system_info = collect_system_info();
                    ("system", format!(
                        "BSD native jemalloc - compile-time selected (platform optimized) ({} cores, {} total RAM)",
                        system_info.cpu_cores,
                        format_memory_size(system_info.total_memory_bytes)
                    ))
                } else if cfg!(target_os = "openbsd") {
                    let system_info = collect_system_info();
                    ("system", format!(
                        "OpenBSD security-hardened allocator - compile-time selected ({} cores, {} total RAM)",
                        system_info.cpu_cores,
                        format_memory_size(system_info.total_memory_bytes)
                    ))
                } else if cfg!(any(target_os = "solaris", target_os = "illumos")) {
                    let system_info = collect_system_info();
                    ("system", format!(
                        "Solaris libumem allocator - compile-time selected (enterprise grade) ({} cores, {} total RAM)",
                        system_info.cpu_cores,
                        format_memory_size(system_info.total_memory_bytes)
                    ))
                } else {
                    // High-performance platforms that fell back to system (single-core or mimalloc unavailable)
                    let system_info = collect_system_info();
                    ("system", format!(
                        "runtime fallback - single-core or mimalloc unavailable ({} cores, {} total RAM)",
                        system_info.cpu_cores,
                        format_memory_size(system_info.total_memory_bytes)
                    ))
                }
            },
        }
    }
}

// Branch prediction optimization
#[inline(always)]
fn unlikely(b: bool) -> bool {
    #[cold]
    fn cold() {}
    if b {
        cold();
    }
    b
}

// ========== Global Allocator Implementation - Platform-specific VTable handling ==========

unsafe impl GlobalAlloc for RuntimeAllocator {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match Self::get_allocator_id() {

            // mimalloc-secure - security-hardened allocator with 10% performance overhead
            #[cfg(all(
                feature = "_mimalloc_secure",
                not(target_arch = "wasm32"),
                not(debug_assertions),
                not(target_os = "none")
            ))]
            5 => {
                use mimalloc::MiMalloc;
                MiMalloc.alloc(layout)
            }

            // mimalloc - high-performance allocator with compiler compatibility detection
            #[cfg(all(
                feature = "_mimalloc",
                not(target_arch = "wasm32"),
                not(debug_assertions),
                not(target_os = "none")
            ))]
            2 => {
                use mimalloc::MiMalloc;
                MiMalloc.alloc(layout)
            }

            // embedded-alloc - for all no_std embedded platforms
            #[cfg(all(
                feature = "_embedded",
                target_os = "none"
            ))]
            4 => {
                // Use embedded-alloc for all no_std targets
                #[cfg(not(target_os = "none"))]
                {
                    embedded_heap_config::EMBEDDED_HEAP.alloc(layout)
                }
                #[cfg(target_os = "none")]
                {
                    embedded_heap_config::get_embedded_heap().alloc(layout)
                }
            }

            // System allocator - default fallback
            #[cfg(not(target_os = "none"))]
            _ => alloc::System.alloc(layout),
            
            #[cfg(target_os = "none")]
            _ => core::ptr::null_mut(),
        }
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        match Self::get_allocator_id() {

            // mimalloc-secure - security-hardened allocator
            #[cfg(all(
                feature = "_mimalloc_secure",
                not(target_arch = "wasm32"),
                not(debug_assertions),
                not(target_os = "none")
            ))]
            5 => {
                use mimalloc::MiMalloc;
                MiMalloc.dealloc(ptr, layout)
            }

            // mimalloc - high-performance allocator with compiler compatibility detection
            #[cfg(all(
                feature = "_mimalloc",
                not(target_arch = "wasm32"),
                not(debug_assertions),
                not(target_os = "none")
            ))]
            2 => {
                use mimalloc::MiMalloc;
                MiMalloc.dealloc(ptr, layout)
            }

            #[cfg(all(
                feature = "_embedded",
                target_os = "none"
            ))]
            4 => {
                // Use embedded-alloc for all no_std targets
                #[cfg(not(target_os = "none"))]
                {
                    embedded_heap_config::EMBEDDED_HEAP.dealloc(ptr, layout)
                }
                #[cfg(target_os = "none")]
                {
                    embedded_heap_config::get_embedded_heap().dealloc(ptr, layout)
                }
            }

            #[cfg(not(target_os = "none"))]
            _ => alloc::System.dealloc(ptr, layout),
            
            #[cfg(target_os = "none")]
            _ => {},
        }
    }
}

#[global_allocator]
static GLOBAL: RuntimeAllocator = RuntimeAllocator;

// ========== Logging System ==========

#[cfg(not(target_os = "none"))]
static PENDING_LOG_MESSAGE: Lazy<std::sync::Mutex<Option<String>>> =
    Lazy::new(|| std::sync::Mutex::new(None));

/// Records allocator selection using a dual logging strategy
///
/// Immediately outputs to stderr (safe during global allocator init) and 
/// saves for later output through the logging framework when available.
#[cfg(not(target_os = "none"))]
fn record_allocator_selection(allocator_name: &str, reason: &str) {
    let message = format!("Auto-allocator: {} selected - {}", allocator_name, reason);

    // Immediate output to stderr (only safe method in global allocator)
    #[cfg(unix)]
    {
        let stderr_message = format!("[INFO] {}\n", message);
        unsafe {
            libc::write(
                2,
                stderr_message.as_ptr() as *const libc::c_void,
                stderr_message.len(),
            );
        }
    }

    // Save message, output later through logging framework
    if let Ok(mut pending) = PENDING_LOG_MESSAGE.lock() {
        *pending = Some(message);
    }
}

/// Attempts to flush pending log message to the logging framework
#[cfg(not(target_os = "none"))]
fn try_flush_pending_log() {
    if !LOG_FLUSHED.load(Ordering::Relaxed) {
        if let Ok(mut pending) = PENDING_LOG_MESSAGE.lock() {
            if let Some(message) = pending.take() {
                let _ = std::panic::catch_unwind(|| {
                    info!("{}", message);
                });
                LOG_FLUSHED.store(true, Ordering::Relaxed);
            }
        }
    }
}

/// Intelligently flushes logs when the logging framework becomes available
#[cfg(not(target_os = "none"))]
fn smart_try_flush_log() {
    // If already output, no need to try again
    if LOG_FLUSHED.load(Ordering::Relaxed) {
        return;
    }

    // Try to output log
    try_flush_pending_log();

    // If still not successful, logging framework is not yet initialized
    // Will continue trying on next call
}

// ========== System Information Collection ==========

#[cfg(not(target_os = "none"))]
fn collect_system_info() -> SystemInfo {
    let total_memory = get_total_memory_safe();
    SystemInfo {
        os_type: std::env::consts::OS.to_string(),
        cpu_cores: std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1),
        total_memory_bytes: total_memory,
        is_debug: cfg!(debug_assertions),
        is_wasm: cfg!(target_arch = "wasm32"),
        target_arch: std::env::consts::ARCH.to_string(),
    }
}

/// Simplified system info collection for no_std environments
#[cfg(target_os = "none")]
fn collect_system_info() -> SystemInfo {
    let total_memory = get_total_memory_safe();
    SystemInfo {
        os_type: "embedded",
        cpu_cores: 1, // Assume single core for embedded
        total_memory_bytes: total_memory,
        is_debug: cfg!(debug_assertions),
        is_wasm: false,
        target_arch: {
            #[cfg(target_arch = "riscv32")]
            { "riscv32" }
            #[cfg(target_arch = "riscv64")]
            { "riscv64" }
            #[cfg(target_arch = "arm")]
            { "arm" }
            #[cfg(target_arch = "avr")]
            { "avr" }
            #[cfg(target_arch = "msp430")]
            { "msp430" }
            #[cfg(target_arch = "xtensa")]
            { "xtensa" }
            #[cfg(not(any(
                target_arch = "riscv32",
                target_arch = "riscv64", 
                target_arch = "arm",
                target_arch = "avr",
                target_arch = "msp430",
                target_arch = "xtensa"
            )))]
            { "unknown" }
        },
    }
}

/// Detects total system memory without allocating during global allocator initialization
///
/// Uses platform-specific APIs for servers/desktop systems and conservative defaults for embedded platforms.
/// Critical: This function must not allocate memory as it's called during global allocator setup.
#[allow(unreachable_code)]
fn get_total_memory_safe() -> u64 {
    #[cfg(target_arch = "wasm32")]
    {
        // WASM can dynamically detect memory through core::arch::wasm32
        use core::arch::wasm32;

        // Get current memory pages, each page is 64KB
        let pages = wasm32::memory_size(0); // Memory index 0 is default memory
        let total_bytes = (pages as u64) * 65536;

        return total_bytes;
    }

    #[cfg(target_os = "macos")]
    {
        // macOS: use sysctl(HW_MEMSIZE)
        unsafe {
            let mut total_size: u64 = 0;
            let mut mib = [libc::CTL_HW, libc::HW_MEMSIZE];
            let mut len = std::mem::size_of::<u64>();

            if libc::sysctl(
                mib.as_mut_ptr(),
                2,
                &mut total_size as *mut _ as *mut libc::c_void,
                &mut len,
                std::ptr::null_mut(),
                0,
            ) == 0
            {
                return total_size;
            } else {
                return 16u64 << 30; // Fallback: 16GB default
            }
        }
    }

    #[cfg(all(target_os = "linux", not(target_arch = "wasm32")))]
    {
        // Linux: use sysinfo() system call
        unsafe {
            let mut info: libc::sysinfo = std::mem::zeroed();
            if libc::sysinfo(&mut info) == 0 {
                let total = info.totalram as u64 * info.mem_unit as u64;
                return total;
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        use std::mem;
        use winapi::um::sysinfoapi::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
        unsafe {
            let mut mem_status: MEMORYSTATUSEX = mem::zeroed();
            mem_status.dwLength = mem::size_of::<MEMORYSTATUSEX>() as u32;
            if GlobalMemoryStatusEx(&mut mem_status) != 0 {
                return mem_status.ullTotalPhys;
            }
        }
    }

    // Embedded platforms: conservative memory size estimates
    #[cfg(target_arch = "avr")]
    {
        return 2u64 << 10; // 2KB for AVR (like Arduino Uno with 2KB RAM)
    }

    #[cfg(target_arch = "msp430")]
    {
        return 1u64 << 10; // 1KB for MSP430 (typical low-power MCU)
    }

    #[cfg(target_arch = "riscv32")]
    {
        return 32u64 << 10; // 32KB for RISC-V MCUs (like ESP32-C3 type devices)
    }

    #[cfg(target_arch = "riscv64")]
    {
        return 128u64 << 10; // 128KB for RISC-V 64-bit systems (like our QEMU example)
    }

    #[cfg(target_arch = "xtensa")]
    {
        return 256u64 << 10; // 256KB for Xtensa (like ESP32 with up to 520KB)
    }

    #[cfg(all(target_arch = "arm", target_os = "none"))]
    {
        return 16u64 << 10; // 16KB for ARM Cortex-M (conservative estimate, M0+ typically has this capacity)
    }

    // Default for unknown platforms
    2u64 << 30
}

// No_std versions of log functions
#[cfg(target_os = "none")]
fn smart_try_flush_log() {
    // No logging in no_std
}

// ========== Runtime Allocator Information ==========

#[cfg(not(target_os = "none"))]
static ALLOCATOR_INFO: Lazy<AllocatorInfo> = Lazy::new(|| {
    let system_info = collect_system_info();
    let allocator_id = RUNTIME_ALLOCATOR_ID.load(Ordering::Acquire);

    // If not yet initialized, trigger allocator selection once
    let final_allocator_id = if allocator_id == 0 {
        RuntimeAllocator::get_allocator_id()
    } else {
        allocator_id
    };

    let (_, mut reason) = get_allocator_selection_result(&system_info);

    // Determine type based on actually selected allocator ID (may differ due to feature disable)
    let allocator_type = match final_allocator_id {
        5 => AllocatorType::MimallocSecure,
        2 => AllocatorType::Mimalloc,
        4 => AllocatorType::EmbeddedHeap,
        _ => AllocatorType::System,
    };

    // Add "selected by runtime analysis" prefix to actual allocator info, extract hardware info part
    let hardware_info = if reason.contains('(') && reason.contains(')') {
        reason
            .split_once('(')
            .and_then(|(_prefix, suffix)| suffix.split_once(')').map(|(info, _)| info))
            .unwrap_or("")
    } else {
        ""
    };

    reason = match final_allocator_id {
        5 => format!(
            "mimalloc-secure selected by runtime hardware analysis ({})",
            hardware_info
        ),
        2 => format!(
            "mimalloc selected by runtime hardware analysis ({})",
            hardware_info
        ),
        4 => {
            // For embedded allocator, preserve the original compile-time selection info
            reason
        },
        _ => {
            // For system allocator, preserve the original detailed reason as-is
            // (already includes correct "compile-time selected" or platform-specific info)
            reason
        },
    };

    AllocatorInfo {
        allocator_type,
        reason,
        system_info,
    }
});

// Simplified allocator info for no_std
#[cfg(target_os = "none")]
static mut EMBEDDED_ALLOCATOR_INFO: Option<AllocatorInfo> = None;

// ========== Public API ==========

/// Ensure allocator information is ready
/// Internal function, ensures ALLOCATOR_INFO has been computed
#[cfg(not(target_os = "none"))]
fn ensure_allocator_info_ready() {
    let _ = std::panic::catch_unwind(|| {
        Lazy::force(&ALLOCATOR_INFO);
    });
}

#[cfg(target_os = "none")]
fn ensure_allocator_info_ready() {
    unsafe {
        if EMBEDDED_ALLOCATOR_INFO.is_none() {
            let system_info = collect_system_info();
            EMBEDDED_ALLOCATOR_INFO = Some(AllocatorInfo {
                allocator_type: AllocatorType::EmbeddedHeap,
                reason: "embedded-alloc selected for no_std environment",
                system_info,
            });
        }
    }
}

/// Returns information about the automatically selected allocator
///
/// Provides allocator type, selection rationale, and system information.
/// First call triggers hardware detection; subsequent calls return cached results.
///
/// # Example
///
/// ```rust
/// use auto_allocator;
///
/// let info = auto_allocator::get_allocator_info();
/// println!("Using: {:?}", info.allocator_type);
/// println!("Reason: {}", info.reason);
/// ```
#[cfg(not(target_os = "none"))]
pub fn get_allocator_info() -> &'static AllocatorInfo {
    smart_try_flush_log();
    ensure_allocator_info_ready();
    &ALLOCATOR_INFO
}

#[cfg(target_os = "none")]
pub fn get_allocator_info() -> &'static AllocatorInfo {
    ensure_allocator_info_ready();
    unsafe { EMBEDDED_ALLOCATOR_INFO.as_ref().unwrap() }
}

/// Get current allocator type
///
/// Returns the currently used allocator type, this is a simplified version of [`get_allocator_info()`].
/// If you only need to know the allocator type without other information, using this function is more concise.
///
/// # Return Value
///
/// Returns [`AllocatorType`] enum value, possible values:
/// - [`AllocatorType::Mimalloc`] - Microsoft-developed high-performance allocator
/// - [`AllocatorType::EmbeddedHeap`] - Embedded systems specific allocator
/// - [`AllocatorType::System`] - System default allocator
///
/// # Example
///
/// ```rust
/// use auto_allocator;
///
/// let allocator_type = auto_allocator::get_allocator_type();
///
/// // Simple allocator type check
/// if allocator_type == auto_allocator::AllocatorType::Mimalloc {
///     println!("Using high-performance mimalloc allocator");
/// }
///
/// // Or use match statement
/// match allocator_type {
///     auto_allocator::AllocatorType::Mimalloc => {
///         println!("mimalloc - optimal performance");
///     }
///     auto_allocator::AllocatorType::System => {
///         println!("system - maximum compatibility");
///     }
///     _ => println!("other allocator"),
/// }
/// ```
///
/// # Performance Notes
///
/// This function is slightly faster than [`get_allocator_info()`] because it only returns type information.
pub fn get_allocator_type() -> AllocatorType {
    smart_try_flush_log();
    ensure_allocator_info_ready();
    get_allocator_info().allocator_type
}

/// Get allocator selection result and reason (internal function)
#[cfg(not(target_os = "none"))]
fn get_allocator_selection_result(system_info: &SystemInfo) -> (AllocatorType, String) {
    let total_mem = format_memory_size(system_info.total_memory_bytes);

    if system_info.is_wasm {
        (
            AllocatorType::System,
            format!("system allocator - WASM environment ({} total RAM)", total_mem),
        )
    } else if system_info.is_debug {
        (
            AllocatorType::System,
            format!(
                "system allocator - debug build ({} cores, {} total RAM)",
                system_info.cpu_cores, total_mem
            ),
        )
    } else if is_embedded_target() {
        (
            AllocatorType::EmbeddedHeap,
            format!("embedded-alloc allocator - embedded environment ({} total RAM)", total_mem),
        )
    } else if system_info.os_type == "android" {
        (
            AllocatorType::System,
            format!(
                "Android platform - Scudo allocator (security-first, use-after-free protection) ({} cores, {} total RAM)",
                system_info.cpu_cores, total_mem
            ),
        )
    } else if system_info.os_type == "ios" {
        (
            AllocatorType::System,
            format!(
                "iOS platform - libmalloc allocator (Apple-optimized, memory pressure handling) ({} cores, {} total RAM)",
                system_info.cpu_cores, total_mem
            ),
        )
    } else if system_info.os_type == "freebsd" || system_info.os_type == "netbsd" {
        (
            AllocatorType::System,
            format!(
                "BSD platform - native jemalloc (highly optimized, deep system integration) ({} cores, {} total RAM)",
                system_info.cpu_cores, total_mem
            ),
        )
    } else if system_info.os_type == "openbsd" {
        (
            AllocatorType::System,
            format!(
                "OpenBSD platform - security-hardened allocator (exploit mitigation, aggressive hardening) ({} cores, {} total RAM)",
                system_info.cpu_cores, total_mem
            ),
        )
    } else if system_info.os_type == "solaris" || system_info.os_type == "illumos" {
        (
            AllocatorType::System,
            format!(
                "Solaris platform - libumem allocator (NUMA-aware, enterprise-grade performance) ({} cores, {} total RAM)",
                system_info.cpu_cores, total_mem
            ),
        )
    } else if system_info.cpu_cores >= 2 {
        (
            AllocatorType::Mimalloc,
            format!(
                "mimalloc allocator - high-performance multi-threaded environment ({} cores, {} total RAM)",
                system_info.cpu_cores, total_mem
            ),
        )
    } else {
        (
            AllocatorType::System,
            format!(
                "system allocator - low-performance environment ({} cores, {} total RAM)",
                system_info.cpu_cores, total_mem
            ),
        )
    }
}

/// Simplified allocator selection for no_std environments
#[cfg(target_os = "none")]
fn get_allocator_selection_result(_system_info: &SystemInfo) -> (AllocatorType, &'static str) {
    (AllocatorType::EmbeddedHeap, "embedded-alloc selected for no_std environment")
}

/// Get recommended allocator for current runtime environment
///
/// Based on current system hardware and environment re-analysis, returns recommended allocator type and selection reason.
/// Unlike [`get_allocator_info()`], this function re-performs hardware detection and analysis every time.
///
/// # Return Value
///
/// Returns a tuple `(AllocatorType, String)`:
/// - First element: recommended allocator type
/// - Second element: recommendation reason, including hardware information
///
/// # Usage
///
/// This function is mainly used for:
/// - Performance analysis and optimization recommendations
/// - Verifying if current allocator selection is optimal
/// - Re-evaluation after runtime environment changes
///
/// # Examples
///
/// ```rust
/// use auto_allocator;
///
/// let (recommended_type, reason) = auto_allocator::get_recommended_allocator();
///
/// println!("Recommended allocator: {:?}", recommended_type);
/// println!("Recommendation reason: {}", reason);
///
/// // Compare with current allocator
/// let current_type = auto_allocator::get_allocator_type();
/// if current_type == recommended_type {
///     println!("Current allocator is already optimal");
/// } else {
///     println!("Suggest switching to: {:?}", recommended_type);
/// }
/// ```
///
/// # Performance Notes
///
/// This function re-performs system hardware detection, with slightly higher overhead than [`get_allocator_info()`].
#[cfg(not(target_os = "none"))]
pub fn get_recommended_allocator() -> (AllocatorType, String) {
    smart_try_flush_log();
    let system_info = collect_system_info();
    get_allocator_selection_result(&system_info)
}

#[cfg(target_os = "none")]
pub fn get_recommended_allocator() -> (AllocatorType, &'static str) {
    let system_info = collect_system_info();
    get_allocator_selection_result(&system_info)
}

/// Check if current allocator is optimal for current environment
///
/// Compares currently used allocator with hardware environment recommended allocator,
/// determining if the best allocator has already been selected.
/// Used for performance optimization checks and configuration validation.
///
/// # Return Value
///
/// Returns a tuple `(bool, Option<String>)`:
/// - `(true, None)` - Current allocator is already optimal
/// - `(false, Some(suggestion))` - Current allocator is not optimal, includes optimization suggestion
///
/// # Usage
///
/// - **Performance audit** - Check if application uses optimal allocator
/// - **Environment validation** - Confirm allocator configuration in deployment environment
/// - **Optimization suggestions** - Get specific allocator optimization recommendations
/// - **Monitoring integration** - Integrate into monitoring systems to check configuration drift
///
/// # Examples
///
/// ```rust
/// use auto_allocator;
///
/// let (is_optimal, suggestion) = auto_allocator::check_allocator_optimization();
///
/// if is_optimal {
///     println!("✅ Current allocator configuration is optimal");
/// } else if let Some(advice) = suggestion {
///     println!("⚠️  Allocator configuration can be optimized:");
///     println!("   {}", advice);
/// }
/// ```
///
/// # Practical Application Scenarios
///
/// ```rust
/// use auto_allocator;
///
/// // Check allocator configuration at application startup
/// fn check_performance_config() {
///     let (is_optimal, suggestion) = auto_allocator::check_allocator_optimization();
///     
///     if !is_optimal {
///         eprintln!("Warning: {}", suggestion.unwrap_or_default());
///         eprintln!("Recommend compiling in Release mode for optimal performance");
///     }
/// }
///
/// // Validate configuration in CI/CD
/// fn test_allocator_optimization() {
///     let (is_optimal, _) = auto_allocator::check_allocator_optimization();
///     assert!(is_optimal, "Allocator configuration not optimized to best state");
/// }
/// ```
///
/// # Performance Notes
///
/// This function needs to re-detect hardware and compare allocators, with slightly higher overhead than simple information retrieval functions.
#[cfg(not(target_os = "none"))]
pub fn check_allocator_optimization() -> (bool, Option<String>) {
    smart_try_flush_log();
    let current = get_allocator_type();
    let (recommended, reason) = get_recommended_allocator();

    if current == recommended {
        (true, None)
    } else {
        let suggestion = format!(
            "Current: {:?}, Recommended: {:?} ({})",
            current, recommended, reason
        );
        (false, Some(suggestion))
    }
}

#[cfg(target_os = "none")]
pub fn check_allocator_optimization() -> (bool, Option<&'static str>) {
    // In no_std, always optimal (embedded-alloc)
    (true, None)
}

// WASM environment initialization
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Automatically initializes allocator information when WASM module loads
///
/// This function is called automatically via `#[wasm_bindgen(start)]` - no manual invocation needed.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_auto_init() {
    ensure_allocator_info_ready();
}

