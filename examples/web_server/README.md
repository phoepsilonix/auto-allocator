# Web Server Example

## 📖 Overview

This example demonstrates how to use auto-allocator in high-performance web server scenarios. It shows how the automatic allocator selects the most suitable memory allocator for server workloads when handling large numbers of concurrent connections, user sessions, and request caching.

## 🎯 Use Cases

- **High-performance web servers** - Need to handle large numbers of concurrent requests
- **API services** - Require fast response times and high throughput
- **Microservice architectures** - Need optimized memory allocation performance
- **Real-time applications** - WebSocket, chat applications, etc.
- **Content management systems** - Need to cache large amounts of data

## 🚀 How to Run

### Basic Usage

```bash
# Debug mode - uses system allocator
cargo run --example web_server

# Release mode - automatically selects high-performance allocator
cargo run --release --example web_server
```

### Performance Comparison Tests

```bash
# Compare performance between different modes
time cargo run --example web_server           # Debug mode
time cargo run --release --example web_server # Release mode
```

### Stress Testing Environment

```bash
# Simulate high-load environment (run multiple times to observe consistency)
for i in {1..5}; do cargo run --release --example web_server; done
```

## 📊 Expected Output

### Release Mode Output Example (Recommended for Production)

```
Web Server Demo
===============
Selected allocator: Mimalloc
Reason: mimalloc selected by runtime hardware analysis (16 cores, 128GB total RAM)
System specs: 16 cores, 128GB total RAM

Simulating web server workload...
Created 10000 user sessions
Cached 5000 requests
Memory allocations completed successfully

This demonstrates how auto-allocator automatically selects
the best allocator for high-performance server workloads.
mimalloc automatically selected - great performance for general server workloads!
```

### Debug Mode Output Example

```
Web Server Demo
===============
Selected allocator: System
Reason: system allocator - debug build (16 cores, 128GB total RAM)
System specs: 16 cores, 128GB total RAM

[...same workload simulation...]
system allocator automatically selected - maximum compatibility!
```

## 🔍 Code Analysis

### Core Functionality Demonstration

1. **Automatic high-performance allocator selection**
   ```rust
   use auto_allocator; // One line of code, automatically selects most suitable allocator for servers
   ```

2. **User session management simulation**
   ```rust
   let mut sessions: HashMap<String, UserSession> = HashMap::new();
   for i in 0..10000 {
       let session = UserSession {
           user_id: format!("user_{}", i),
           data: format!("session_data_{}", i).repeat(10),
           timestamp: std::time::SystemTime::now(),
       };
       sessions.insert(format!("session_{}", i), session);
   }
   ```

3. **Request caching simulation**
   ```rust
   let mut request_cache: Vec<RequestData> = Vec::new();
   for i in 0..5000 {
       let request = RequestData {
           id: i,
           url: format!("/api/endpoint/{}", i),
           headers: vec![...],
           body: format!("{{\"data\": \"request_{}\"}}", i),
       };
       request_cache.push(request);
   }
   ```

## 🏆 Performance Advantages

### Why mimalloc is Suitable for Web Servers?

Based on Microsoft and independent research performance benchmarks:

1. **Excellent multi-threaded performance**
   - 1.6x faster than system allocators in high-concurrency scenarios
   - Less lock contention, suitable for server multi-threaded models

2. **Memory efficiency**
   - Better memory locality
   - Reduced memory fragmentation
   - Faster allocation/deallocation speeds

3. **Low-latency characteristics**
   - More predictable allocation times
   - Suitable for response time-sensitive web applications

### Automatic Selection Logic

| Environment Condition | Selected Allocator | Reason |
|----------------------|-------------------|---------|
| Debug mode | System | Fast compilation, development debugging |
| Modern platforms (Release) | **Mimalloc** | **Best server performance** |
| Mobile/WASM platforms | System | Platform compliance |
| Embedded systems | EmbeddedHeap | Resource optimization |

## 📈 Workload Analysis

### Typical Web Server Memory Patterns

1. **Frequent small object allocation**
   - Request parsing, response building
   - JSON serialization/deserialization
   - String operations

2. **Medium-sized cached objects**
   - User session data
   - Template caching
   - Database query results

3. **Large memory allocations**
   - File upload/download
   - Response body building
   - Database connection pools

### mimalloc Advantages in These Scenarios

- **Small object allocation**: Faster allocation speeds, less metadata overhead
- **Medium objects**: Better memory locality, reduced cache misses
- **Large objects**: Efficient large memory block management

## 🔧 Integration into Real Projects

### Actix Web Integration Example

```rust
use auto_allocator; // Add this line

use actix_web::{web, App, HttpServer, Result};

async fn hello() -> Result<&'static str> {
    Ok("Hello, auto-allocator!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

### Tokio Integration Example

```rust
use auto_allocator; // Add this line

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            // Handle connection...automatically uses optimized allocator
        });
    }
}
```

## 💡 Summary

This example demonstrates auto-allocator in web server scenarios - just add `use auto_allocator;` and get optimal memory performance automatically without any configuration.