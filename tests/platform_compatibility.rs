//! Platform compatibility tests for auto-allocator
//!
//! These tests verify that the allocator selection logic works correctly
//! across different platforms and build configurations.

use auto_allocator::get_allocator_info;

#[test]
fn test_allocator_selection_consistency() {
    // Test that allocator selection is consistent across multiple calls
    let info1 = get_allocator_info();
    let info2 = get_allocator_info();

    assert_eq!(info1.allocator_type, info2.allocator_type);
    assert_eq!(info1.reason, info2.reason);
}

#[test]
fn test_platform_specific_allocator_selection() {
    let info = get_allocator_info();

    // Verify platform-specific behavior
    #[cfg(target_arch = "wasm32")]
    {
        // WASM should always use system allocator
        assert_eq!(info.allocator_type, auto_allocator::AllocatorType::System);
        assert!(info.reason.contains("WASM") || info.reason.contains("compatibility"));
    }

    #[cfg(debug_assertions)]
    {
        // Debug builds should prefer system allocator for fast compilation
        assert_eq!(info.allocator_type, auto_allocator::AllocatorType::System);
        assert!(info.reason.contains("Debug") || info.reason.contains("debug"));
    }

    #[cfg(all(
        not(debug_assertions),
        not(target_arch = "wasm32"),
        target_os = "none"
    ))]
    {
        // Embedded targets should use embedded allocator
        assert_eq!(info.allocator_type, auto_allocator::AllocatorType::EmbeddedHeap);
        assert!(info.reason.contains("embedded") || info.reason.contains("Embedded"));
    }

    #[cfg(all(
        not(debug_assertions),
        not(target_arch = "wasm32"),
        target_env = "msvc",
        not(target_os = "none")
    ))]
    {
        // Windows MSVC should use mimalloc or system
        // Test is informational only since jemalloc support was removed
        println!("Windows MSVC allocator: {:?}", info.allocator_type);
    }

    // Test mimalloc selection on supported platforms (release mode, non-embedded, non-WASM)
    #[cfg(all(
        not(debug_assertions),
        not(target_arch = "wasm32"),
        any(
            target_os = "windows",
            target_os = "macos",
            all(target_os = "linux", target_env = "gnu")
        ),
        not(target_os = "none")
    ))]
    {
        // Modern platforms in release mode should use mimalloc for optimal performance
        assert_eq!(info.allocator_type, auto_allocator::AllocatorType::Mimalloc);
        assert!(info.reason.contains("performance") || info.reason.contains("optimal") || info.reason.contains("mimalloc"));
    }
}

#[test]
fn test_memory_allocation_basic() {
    // Test basic memory allocation works with selected allocator
    let data: Vec<u8> = vec![0; 1024];
    assert_eq!(data.len(), 1024);

    let string_data = String::from("Hello, auto-allocator!");
    assert_eq!(string_data, "Hello, auto-allocator!");
}

#[test]
fn test_memory_allocation_stress() {
    // Stress test memory allocation
    let mut allocations = Vec::new();

    // Allocate various sizes
    for size in [16, 64, 256, 1024, 4096] {
        let data: Vec<u8> = vec![42; size];
        assert_eq!(data.len(), size);
        assert!(data.iter().all(|&x| x == 42));
        allocations.push(data);
    }

    // Verify all allocations are still valid
    for (i, alloc) in allocations.iter().enumerate() {
        let expected_size = [16, 64, 256, 1024, 4096][i];
        assert_eq!(alloc.len(), expected_size);
        assert!(alloc.iter().all(|&x| x == 42));
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn test_system_info_detection() {
    let info = get_allocator_info();

    // Verify system information is reasonable
    assert!(info.system_info.cpu_cores >= 1);
    assert!(info.system_info.cpu_cores <= 1024); // Reasonable upper bound

    assert!(info.system_info.total_memory_bytes > 0);

    // Verify OS detection
    #[cfg(target_os = "windows")]
    assert_eq!(info.system_info.os_type, "windows");

    #[cfg(target_os = "macos")]
    assert_eq!(info.system_info.os_type, "macos");

    #[cfg(target_os = "linux")]
    assert_eq!(info.system_info.os_type, "linux");

    // Verify debug flag is correct
    #[cfg(debug_assertions)]
    assert!(info.system_info.is_debug);

    #[cfg(not(debug_assertions))]
    assert!(!info.system_info.is_debug);

    // Verify WASM flag
    assert_eq!(info.system_info.is_wasm, cfg!(target_arch = "wasm32"));
}

#[test]
fn test_allocator_optimization_check() {
    let (is_optimal, suggestion) = auto_allocator::check_allocator_optimization();

    // In debug mode, system allocator should be optimal
    #[cfg(debug_assertions)]
    {
        assert!(is_optimal);
        assert!(suggestion.is_none());
    }

    // On WASM, system allocator should be optimal
    #[cfg(target_arch = "wasm32")]
    {
        assert!(is_optimal);
        assert!(suggestion.is_none());
    }

    // On modern platforms with mimalloc, should be optimal
    #[cfg(all(
        not(debug_assertions),
        not(target_arch = "wasm32"),
        any(
            target_os = "windows",
            target_os = "macos", 
            all(target_os = "linux", target_env = "gnu")
        ),
        not(target_os = "none")
    ))]
    {
        assert!(is_optimal);
        assert!(suggestion.is_none());
    }

    // Suggestion should be meaningful if provided
    if let Some(msg) = suggestion {
        assert!(!msg.is_empty());
        assert!(msg.len() > 10); // Should be a meaningful message
    }
}

#[test]
fn test_concurrent_access() {
    use std::thread;

    // Test that allocator works correctly with multiple threads
    let handles: Vec<_> = (0..4)
        .map(|i| {
            thread::spawn(move || {
                let info = get_allocator_info();

                // Each thread should get the same allocator type
                let data: Vec<u8> = vec![i as u8; 1000];
                assert_eq!(data.len(), 1000);
                assert!(data.iter().all(|&x| x == i as u8));

                info.allocator_type
            })
        })
        .collect();

    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // All threads should have selected the same allocator
    let first_allocator = results[0];
    assert!(results
        .iter()
        .all(|&allocator| allocator == first_allocator));
}
