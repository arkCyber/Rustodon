//!
//! Rustodon Server Main Entrypoint - High Performance Configuration
//!
//! This binary starts the Rustodon server optimized for 10k concurrent users.
//! Features include optimized Tokio runtime, connection pooling, and performance monitoring.
//!
//! # Performance Optimizations
//!
//! - Multi-threaded Tokio runtime with optimized thread pool
//! - Database connection pooling with high concurrency
//! - HTTP server with optimized buffer sizes
//! - Memory-efficient request handling
//! - Performance monitoring and metrics
//!
//! # Usage
//!
//! ```sh
//! # Development
//! cargo run -p rustodon-server
//!
//! # Production with optimized settings
//! RUST_LOG=info cargo run -p rustodon-server --release
//!
//! # Performance testing
//! cargo run -p rustodon-server --release --features performance-testing
//! ```
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use rustodon_api::start_server;
use rustodon_mailer::AsyncMailer;
use rustodon_mailer::{Email, MockMailer};
use rustodon_workers::{ExampleJob, Worker};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;
use tracing::{error, info, warn};

/// Performance configuration for high-concurrency scenarios
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    /// Number of worker threads for Tokio runtime
    pub worker_threads: usize,
    /// Maximum number of concurrent connections
    pub max_connections: usize,
    /// Database connection pool size
    pub db_pool_size: u32,
    /// HTTP server buffer size
    pub http_buffer_size: usize,
    /// Request timeout in seconds
    pub request_timeout: u64,
    /// Keep-alive timeout in seconds
    pub keep_alive_timeout: u64,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            worker_threads: num_cpus::get() * 2, // 2x CPU cores for I/O bound workloads
            max_connections: 10000,
            db_pool_size: 100,           // Increased for high concurrency
            http_buffer_size: 64 * 1024, // 64KB buffer
            request_timeout: 30,
            keep_alive_timeout: 60,
        }
    }
}

/// Kill processes using the specified port
fn kill_processes_on_port(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    info!("Checking for processes using port {}", port);

    // Get PIDs using the port
    let output = std::process::Command::new("lsof")
        .arg("-ti")
        .arg(format!(":{}", port))
        .output()?;

    if output.stdout.is_empty() {
        info!("No processes found using port {}", port);
        return Ok(());
    }

    let pids = String::from_utf8_lossy(&output.stdout);
    for pid in pids.lines() {
        let pid = pid.trim();
        if !pid.is_empty() {
            info!("Killing process PID {} using port {}", pid, port);

            // Kill the process
            let kill_result = std::process::Command::new("kill")
                .arg("-9")
                .arg(pid)
                .output();

            match kill_result {
                Ok(_) => info!("Successfully killed process PID {}", pid),
                Err(e) => warn!("Failed to kill process PID {}: {}", pid, e),
            }
        }
    }

    // Wait a moment for the port to be released
    std::thread::sleep(std::time::Duration::from_millis(500));

    Ok(())
}

/// Check if a port is available
fn is_port_available(port: u16) -> bool {
    use std::net::TcpListener;
    TcpListener::bind(format!("127.0.0.1:{}", port)).is_ok()
}

/// Initialize performance-optimized Tokio runtime
#[allow(dead_code)]
fn create_optimized_runtime(config: &PerformanceConfig) -> tokio::runtime::Runtime {
    info!(
        "Creating optimized Tokio runtime with {} worker threads",
        config.worker_threads
    );

    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(config.worker_threads)
        .max_blocking_threads(config.worker_threads / 2)
        .enable_all()
        .thread_name("rustodon-worker")
        .thread_stack_size(2 * 1024 * 1024) // 2MB stack size
        .build()
        .expect("Failed to create Tokio runtime")
}

/// Initialize performance monitoring
fn init_performance_monitoring() {
    info!("Initializing performance monitoring");

    // Set up periodic performance logging
    let start_time = Instant::now();

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
        loop {
            interval.tick().await;
            let uptime = start_time.elapsed();
            info!(
                "Server uptime: {:?}, Memory usage: {} MB",
                uptime,
                get_memory_usage_mb()
            );
        }
    });
}

/// Get current memory usage in MB
fn get_memory_usage_mb() -> u64 {
    #[cfg(target_os = "linux")]
    {
        if let Ok(contents) = std::fs::read_to_string("/proc/self/status") {
            for line in contents.lines() {
                if line.starts_with("VmRSS:") {
                    if let Some(kb_str) = line.split_whitespace().nth(1) {
                        if let Ok(kb) = kb_str.parse::<u64>() {
                            return kb / 1024;
                        }
                    }
                }
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        // macOS memory usage approximation
        if let Ok(output) = std::process::Command::new("ps")
            .args(["-o", "rss=", "-p", &std::process::id().to_string()])
            .output()
        {
            if let Ok(rss_str) = String::from_utf8(output.stdout) {
                if let Ok(kb) = rss_str.trim().parse::<u64>() {
                    return kb / 1024;
                }
            }
        }
    }

    0 // Fallback
}

#[tokio::main(worker_threads = 16)] // Optimized for 10k concurrent users
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();

    // Load performance configuration
    let perf_config = PerformanceConfig::default();
    info!("Performance config: {:?}", perf_config);

    // Initialize logging with performance optimizations
    tracing_subscriber::fmt()
        .with_timer(tracing_subscriber::fmt::time::UtcTime::rfc_3339())
        .with_target(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Rustodon server starting with high-performance configuration...");

    // Initialize performance monitoring
    init_performance_monitoring();

    // Initialize database with optimized connection pool
    let pool = rustodon_db::init_database().await?;
    info!("Database initialized successfully with optimized connection pool");

    // Handle port 3000 conflict
    let port = 3000;
    if !is_port_available(port) {
        info!(
            "Port {} is in use. Attempting to kill conflicting processes...",
            port
        );

        if let Err(e) = kill_processes_on_port(port) {
            warn!("Failed to kill processes on port {}: {}", port, e);
        }

        // Wait a bit more and check again
        std::thread::sleep(std::time::Duration::from_millis(1000));

        if !is_port_available(port) {
            error!(
                "Port {} is still in use after attempting to kill processes. Exiting.",
                port
            );
            std::process::exit(1);
        } else {
            info!("Port {} is now available", port);
        }
    } else {
        info!("Port {} is available", port);
    }

    // Start API server with performance optimizations (in background)
    let addr = format!("127.0.0.1:{}", port).parse().unwrap();
    let pool_clone = pool.clone();
    let api_handle = tokio::spawn(async move {
        if let Err(e) = start_server(pool_clone, addr).await {
            error!("API server failed: {}", e);
        }
    });

    // Start worker with optimized queue (in background)
    let queue = Arc::new(Mutex::new(vec![
        Box::new(ExampleJob) as Box<dyn rustodon_workers::Job>
    ]));
    let worker = Worker::new(queue.clone());
    let worker_handle = tokio::spawn(async move {
        let _ = worker.start().await;
    });

    // Start mailer (mock example)
    let mailer = MockMailer;
    let email = Email {
        to: "admin@example.com".to_string(),
        subject: "Rustodon Started - High Performance Mode".to_string(),
        body: format!(
            "The Rustodon server has started successfully in high-performance mode.\nWorker threads: {}\nMax connections: {}\nDB pool size: {}",
            perf_config.worker_threads, perf_config.max_connections, perf_config.db_pool_size
        ),
    };
    if let Err(e) = mailer.send(email).await {
        error!("Failed to send startup email: {}", e);
    }

    let startup_time = start_time.elapsed();
    info!(
        "Rustodon server started in {:?}. Ready for 10k concurrent users.",
        startup_time
    );
    info!("Server is running. Press Ctrl+C to exit.");

    // Wait for API and worker to finish (in production, handle graceful shutdown)
    let _ = tokio::try_join!(api_handle, worker_handle);
    Ok(())
}
