/// Demonstrates performance-optimized allocator selection using auto-allocator in web servers
#[allow(clippy::single_component_path_imports)]
use auto_allocator;
use std::collections::HashMap;

fn main() {
    // Initialize logging (skip on WASM)
    #[cfg(not(target_arch = "wasm32"))]
    env_logger::init();

    println!("Web Server Demo");
    println!("===============");

    // Get allocator information
    let info = auto_allocator::get_allocator_info();

    println!("Selected allocator: {:?}", info.allocator_type);
    println!("Reason: {}", info.reason);
    println!(
        "System specs: {} cores, {} total RAM",
        info.system_info.cpu_cores,
        auto_allocator::format_memory_size(info.system_info.total_memory_bytes)
    );
    println!();

    // Simulate memory usage in high-performance server environments
    // auto-allocator automatically selects mimalloc for optimal performance (based on Microsoft performance research)
    println!("Simulating web server workload...");

    // Simulate user session storage
    let mut sessions: HashMap<String, UserSession> = HashMap::new();

    // Create many user sessions
    for i in 0..10000 {
        let session = UserSession {
            user_id: format!("user_{}", i),
            data: format!("session_data_{}", i).repeat(10),
            timestamp: std::time::SystemTime::now(),
        };
        sessions.insert(format!("session_{}", i), session);
    }

    // Simulate request processing
    let mut request_cache: Vec<RequestData> = Vec::new();
    for i in 0..5000 {
        let request = RequestData {
            id: i,
            url: format!("/api/endpoint/{}", i),
            headers: vec![
                "Content-Type: application/json".to_string(),
                "User-Agent: AutoAllocator/1.0".to_string(),
                format!("X-Request-ID: {}", i),
            ],
            body: format!("{{\"data\": \"request_{}\"}}", i),
        };
        request_cache.push(request);
    }

    println!("Created {} user sessions", sessions.len());
    println!("Cached {} requests", request_cache.len());
    println!("Memory allocations completed successfully");

    // Display memory usage information
    println!();
    println!("This demonstrates how auto-allocator automatically selects");
    println!("the best allocator for high-performance server workloads.");

    match info.allocator_type {
        auto_allocator::AllocatorType::MimallocSecure => {
            println!("mimalloc-secure automatically selected - security-hardened for server workloads!");
            println!("Security features enabled with ~10% performance overhead");
        }
        auto_allocator::AllocatorType::Mimalloc => {
            println!(
                "mimalloc automatically selected - excellent performance for server workloads!"
            );
        }
        auto_allocator::AllocatorType::System => {
            println!("system allocator automatically selected - maximum compatibility!");
        }
        auto_allocator::AllocatorType::EmbeddedHeap => {
            println!("embedded allocator automatically selected - optimized for constrained environments!");
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)] // Fields would be used in real web servers
struct UserSession {
    user_id: String,
    data: String,
    timestamp: std::time::SystemTime,
}

#[derive(Debug)]
#[allow(dead_code)] // Fields would be used in real web servers
struct RequestData {
    id: u32,
    url: String,
    headers: Vec<String>,
    body: String,
}
