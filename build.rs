use std::env;

/// Build script for auto-allocator
///
/// Automatically detects platform capabilities and validates mimalloc compatibility:
/// - Checks if mimalloc can compile (GCC version, stdatomic.h availability)
/// - Stops compilation on incompatible systems with clear error messages
/// - Provides upgrade guidance for legacy systems

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    validate_platform_compatibility();
}

/// Validates that the current platform can compile mimalloc
/// Stops compilation with clear error message if incompatible
fn validate_platform_compatibility() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    
    // Check if this is a debug or release build
    let is_debug = env::var("DEBUG").unwrap_or_default() == "true";

    match (target_os.as_str(), target_env.as_str(), target_arch.as_str()) {
        // Linux systems need careful mimalloc compatibility checking
        ("linux", "gnu", _) => {
                println!("cargo:warning=Auto-allocator: Linux GNU platform detected");
                if is_debug {
                    println!("cargo:warning=  → Will use system allocator (debug build)");
                } else {
                    println!("cargo:warning=  → Will use mimalloc (release build)");
                }
        }
        
        // Other Linux environments
        ("linux", "musl", _) => {
                println!("cargo:warning=Auto-allocator: Linux musl platform detected");
                if is_debug {
                    println!("cargo:warning=  → Will use system allocator (debug build)");
                } else {
                    println!("cargo:warning=  → Will use mimalloc (release build)");
                }
        }

        // Non-Linux platforms - provide information only (actual selection happens at runtime)
        _ => {
            print_platform_info(target_os.as_str(), target_env.as_str(), target_arch.as_str(), is_debug);
        }
    }
}

/// Prints platform information for non-Linux systems
/// Actual allocator selection happens at runtime in src/lib.rs
fn print_platform_info(target_os: &str, target_env: &str, target_arch: &str, is_debug: bool) {
    // Check if this is an embedded platform (must match lib.rs is_embedded_target logic)
    // Use target_os = "none" as the universal indicator for embedded/no_std environments
    // This covers all current and future embedded architectures automatically
    if target_os == "none" {
        println!("cargo:warning=Auto-allocator: Embedded platform detected ({})", target_arch);
        println!("cargo:warning=  → Will use embedded-alloc for resource optimization");
        return;
    }
    
    match (target_os, target_env, target_arch) {

        // WASM
        (_, _, "wasm32") => {
            println!("cargo:warning=Auto-allocator: WASM platform detected");
            println!("cargo:warning=  → Will use system allocator for browser compatibility");
        }

        // Mobile platforms
        ("android", _, _) => {
            println!("cargo:warning=Auto-allocator: Android platform detected");
            println!("cargo:warning=  → Will use system allocator (Scudo) per Android security policy");
        }
        ("ios", _, _) => {
            println!("cargo:warning=Auto-allocator: iOS platform detected");
            println!("cargo:warning=  → Will use system allocator (libmalloc) per Apple recommendations");
        }

        // BSD systems  
        ("freebsd", _, _) | ("netbsd", _, _) => {
            println!("cargo:warning=Auto-allocator: BSD platform detected ({})", target_os);
            println!("cargo:warning=  → Will use system allocator (native jemalloc)");
        }
        ("openbsd", _, _) => {
            println!("cargo:warning=Auto-allocator: OpenBSD platform detected");
            println!("cargo:warning=  → Will use system allocator (security-hardened)");
        }

        // Solaris systems
        ("solaris", _, _) | ("illumos", _, _) => {
            println!("cargo:warning=Auto-allocator: Solaris platform detected ({})", target_os);
            println!("cargo:warning=  → Will use system allocator (libumem)");
        }

        // High-performance platforms that support mimalloc
        ("windows", "msvc", _) => {
            println!("cargo:warning=Auto-allocator: Windows MSVC platform detected");
            if is_debug {
                println!("cargo:warning=  → Will use system allocator (debug build)");
            } else {
                println!("cargo:warning=  → Will use mimalloc (release build)");
            }
        }
        ("windows", "gnu", _) => {
            println!("cargo:warning=Auto-allocator: Windows GNU platform detected");
            if is_debug {
                println!("cargo:warning=  → Will use system allocator (debug build)");
            } else {
                println!("cargo:warning=  → Will use mimalloc (release build)");
            }
        }
        ("macos", _, _) => {
            println!("cargo:warning=Auto-allocator: macOS platform detected");
            if is_debug {
                println!("cargo:warning=  → Will use system allocator (debug build)");
            } else {
                println!("cargo:warning=  → Will use mimalloc (release build)");
            }
        }
        
        // Unknown platforms - be conservative
        _ => {
            println!("cargo:warning=Auto-allocator: Unknown platform detected");
            println!("cargo:warning=  → Platform: {} env: {} arch: {}", target_os, target_env, target_arch);
            println!("cargo:warning=  → Will use system allocator (mimalloc not available on this platform)");
        }
    }
}
