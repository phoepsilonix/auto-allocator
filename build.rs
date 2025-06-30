use std::env;
use std::process::Command;

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
            if !can_use_mimalloc_on_linux() {
                print_compilation_error_and_exit();
            } else {
                println!("cargo:warning=Auto-allocator: Linux GNU platform detected");
                if is_debug {
                    println!("cargo:warning=  → Will use system allocator (debug build)");
                } else {
                    println!("cargo:warning=  → Will use mimalloc (release build)");
                }
            }
        }
        
        // Other Linux environments
        ("linux", "musl", _) => {
            if !can_use_mimalloc_on_linux() {
                print_compilation_error_and_exit();
            } else {
                println!("cargo:warning=Auto-allocator: Linux musl platform detected");
                if is_debug {
                    println!("cargo:warning=  → Will use system allocator (debug build)");
                } else {
                    println!("cargo:warning=  → Will use mimalloc (release build)");
                }
            }
        }

        // Non-Linux platforms - provide information only (actual selection happens at runtime)
        _ => {
            print_platform_info(target_os.as_str(), target_env.as_str(), target_arch.as_str(), is_debug);
        }
    }
}

/// Checks if mimalloc can be compiled on this Linux system
/// Returns false for old GCC versions (4.8.x) that lack stdatomic.h
fn can_use_mimalloc_on_linux() -> bool {
    // First check if we have stdatomic.h available
    if !has_stdatomic_header() {
        return false;
    }

    // Check GCC version if available
    if let Some(gcc_version) = get_gcc_version() {
        if gcc_version < 49 {  // GCC 4.9+ required for reliable stdatomic.h
            return false;
        }
    }

    true
}

/// Checks if stdatomic.h is available by trying to compile a test program
fn has_stdatomic_header() -> bool {
    let test_program = r#"
        #include <stdatomic.h>
        int main() {
            atomic_int x = ATOMIC_VAR_INIT(0);
            return atomic_load(&x);
        }
    "#;

    // Try to compile the test program
    match std::process::Command::new("cc")
        .arg("-c")
        .arg("-o")
        .arg("/dev/null")
        .arg("-x")
        .arg("c")
        .arg("-")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
    {
        Ok(mut child) => {
            if let Some(mut stdin) = child.stdin.take() {
                use std::io::Write;
                let _ = stdin.write_all(test_program.as_bytes());
            }
            child.wait().map(|status| status.success()).unwrap_or(false)
        }
        Err(_) => false, // cc not available
    }
}

/// Prints compilation error and exits the build process
fn print_compilation_error_and_exit() {
    eprintln!();
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("❌ AUTO-ALLOCATOR COMPILATION ERROR");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!();
    eprintln!("🚫 Cannot compile on this system: mimalloc requires modern GCC with stdatomic.h");
    eprintln!();
    
    if let Some(gcc_version) = get_gcc_version() {
        eprintln!("📊 System Information:");
        eprintln!("   • Detected GCC version: {}.x", gcc_version / 10);
        eprintln!("   • Required GCC version: 4.9+");
        eprintln!("   • stdatomic.h available: {}", if has_stdatomic_header() { "✅ Yes" } else { "❌ No" });
    } else {
        eprintln!("📊 System Information:");
        eprintln!("   • GCC compiler: Not detected or not available");
        eprintln!("   • stdatomic.h available: {}", if has_stdatomic_header() { "✅ Yes" } else { "❌ No" });
    }
    
    eprintln!();
    eprintln!("ℹ️  Note: You may see additional 'libmimalloc-sys' compilation errors below.");
    eprintln!("   This is expected - the same stdatomic.h issue affects mimalloc compilation.");
    eprintln!();
    eprintln!("💡 Solutions to fix this issue:");
    eprintln!();
    eprintln!("   🔧 Option 1 - Upgrade GCC (Recommended):");
    eprintln!("      CentOS 7: sudo yum install -y centos-release-scl devtoolset-11-gcc");
    eprintln!("      Then: source /opt/rh/devtoolset-11/enable");
    eprintln!("      Ubuntu: sudo apt-get install gcc-9");
    eprintln!();
    eprintln!("   🔄 Option 2 - Upgrade OS:");
    eprintln!("      CentOS 7 → CentOS 8+ / RHEL 8+ / Rocky Linux 8+");
    eprintln!("      Ubuntu 16.04 → Ubuntu 18.04+");
    eprintln!();
    eprintln!("   ⚡ Option 3 - Use system allocator:");
    eprintln!("      Remove auto-allocator and use Rust's default system allocator");
    eprintln!("      (No performance benefits, but maximum compatibility)");
    eprintln!();
    eprintln!("🎯 Why this matters:");
    eprintln!("   auto-allocator provides 1.6x faster allocation performance, but requires");
    eprintln!("   modern compiler support. Legacy systems cannot benefit from this optimization.");
    eprintln!();
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    std::process::exit(1);
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

/// Attempts to detect GCC version by running the compiler
fn get_gcc_version() -> Option<u32> {
    // Check if we're actually using GCC
    if let Ok(cc) = env::var("CC") {
        if !cc.contains("gcc") {
            return None;
        }
    }
    let gcc_cmd = env::var("CC").unwrap_or_else(|_| "gcc".to_string());
    let output = Command::new(&gcc_cmd).arg("--version").output().ok()?;

    let version_str = String::from_utf8(output.stdout).ok()?;
    for line in version_str.lines() {
        if line.to_lowercase().contains("gcc") {
            for part in line.split_whitespace() {
                if let Some(version) = parse_version_number(part) {
                    return Some(version);
                }
            }
        }
    }
    None
}

/// Extracts major version number from version string (e.g., "7.5.0" -> 75)
fn parse_version_number(s: &str) -> Option<u32> {
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() >= 2 {
        if let (Ok(major), Ok(minor)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
            return Some(major * 10 + minor);
        }
    }
    parts.first()?.parse::<u32>().ok().map(|v| v * 10)
}