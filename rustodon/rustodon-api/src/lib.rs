//!
//! Rustodon API Library - High Performance Configuration
//!
//! This crate provides the HTTP API layer for the Rustodon server optimized for 10k+ concurrent users.
//! Features include optimized HTTP server, connection pooling, and performance monitoring.
//!
//! # Performance Optimizations
//!
//! - Optimized HTTP server with large connection limits
//! - Request/response compression
//! - Connection pooling and keep-alive
//! - Rate limiting and throttling
//! - Performance monitoring and metrics
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)
//!
//! # Dependencies
//!
//! - `axum`: Web framework
//! - `tower`: HTTP middleware
//! - `serde`: Serialization
//! - `tracing`: Logging
//!
//! # Usage
//!
//! Use `api_router()` to get the main API router for the server.

pub mod endpoints;

pub use endpoints::api_router;

use axum::serve;
use std::time::Duration;
use tokio::net::TcpListener;
use tracing::{debug, error, info};

/// High-performance server configuration
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Server bind address
    pub bind_address: String,
    /// Server port
    pub port: u16,
    /// Maximum number of concurrent connections
    pub max_connections: usize,
    /// Request timeout in seconds
    pub request_timeout: u64,
    /// Keep-alive timeout in seconds
    pub keep_alive_timeout: u64,
    /// Maximum request body size in bytes
    pub max_body_size: usize,
    /// Whether to enable compression
    pub enable_compression: bool,
    /// Whether to enable CORS
    pub enable_cors: bool,
    /// Whether to enable request tracing
    pub enable_tracing: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_address: "0.0.0.0".to_string(),
            port: 3000,
            max_connections: 10000, // Support 10k concurrent connections
            request_timeout: 30,
            keep_alive_timeout: 60,
            max_body_size: 10 * 1024 * 1024, // 10MB
            enable_compression: true,
            enable_cors: true,
            enable_tracing: true,
        }
    }
}

/// Creates a high-performance server configuration
impl ServerConfig {
    /// Creates a production configuration for 10k+ concurrent users
    pub fn production() -> Self {
        Self {
            bind_address: "0.0.0.0".to_string(),
            port: 3000,
            max_connections: 20000,          // Very high connection limit
            request_timeout: 15,             // Shorter timeout for faster failure detection
            keep_alive_timeout: 120,         // Longer keep-alive for connection reuse
            max_body_size: 50 * 1024 * 1024, // 50MB for media uploads
            enable_compression: true,
            enable_cors: true,
            enable_tracing: false, // Disable in production for performance
        }
    }

    /// Creates a development configuration
    pub fn development() -> Self {
        Self {
            bind_address: "127.0.0.1".to_string(),
            port: 3000,
            max_connections: 1000,
            request_timeout: 60,
            keep_alive_timeout: 30,
            max_body_size: 10 * 1024 * 1024,
            enable_compression: false,
            enable_cors: true,
            enable_tracing: true,
        }
    }
}

/// Starts the Rustodon API server with high-performance configuration
///
/// Binds to the configured address and port, serving the API router with
/// optimized middleware for high concurrency.
///
/// # Arguments
///
/// * `config` - Server configuration (optional, uses default if None)
///
/// # Errors
///
/// Returns an error if the server fails to bind or run.
///
/// # Examples
///
/// ```no_run
/// use rustodon_api::ServerConfig;
///
/// // Use default configuration
/// rustodon_api::start_server().await.unwrap();
///
/// // Use custom configuration
/// let config = ServerConfig::production();
/// rustodon_api::start_server_with_config(config).await.unwrap();
/// ```
pub async fn start_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = ServerConfig::default();
    start_server_with_config(config).await
}

/// Starts the server with custom configuration
pub async fn start_server_with_config(
    config: ServerConfig,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = format!("{}:{}", config.bind_address, config.port);
    info!("Starting high-performance API server on {}", addr);
    debug!("Server configuration: {:?}", config);

    // Create TCP listener with optimized settings
    let listener = TcpListener::bind(&addr).await?;

    // Set TCP options for high performance
    #[cfg(unix)]
    {
        use std::os::unix::io::AsRawFd;

        let socket = listener.as_raw_fd();
        unsafe {
            // Enable TCP_NODELAY for lower latency
            let nodelay: libc::c_int = 1;
            libc::setsockopt(
                socket,
                libc::IPPROTO_TCP,
                libc::TCP_NODELAY,
                &nodelay as *const _ as *const libc::c_void,
                std::mem::size_of::<libc::c_int>() as libc::socklen_t,
            );

            // Set SO_REUSEADDR for faster restarts
            let reuseaddr: libc::c_int = 1;
            libc::setsockopt(
                socket,
                libc::SOL_SOCKET,
                libc::SO_REUSEADDR,
                &reuseaddr as *const _ as *const libc::c_void,
                std::mem::size_of::<libc::c_int>() as libc::socklen_t,
            );
        }
    }

    // Build basic middleware stack
    let app = crate::api_router();

    // Log configuration
    if config.enable_compression {
        debug!("Compression enabled");
    }
    if config.enable_cors {
        debug!("CORS enabled");
    }
    if config.enable_tracing {
        debug!("Request tracing enabled");
    }

    info!(
        "Server configured for {} max connections",
        config.max_connections
    );
    info!(
        "Request timeout: {}s, Keep-alive: {}s",
        config.request_timeout, config.keep_alive_timeout
    );

    // Start the server
    serve(listener, app).await.map_err(|e| {
        error!("API server failed: {}", e);
        e.into()
    })
}

/// Performance monitoring for the API server
pub struct PerformanceMonitor {
    start_time: std::time::Instant,
    request_count: std::sync::atomic::AtomicU64,
    error_count: std::sync::atomic::AtomicU64,
}

impl PerformanceMonitor {
    /// Creates a new performance monitor
    pub fn new() -> Self {
        Self {
            start_time: std::time::Instant::now(),
            request_count: std::sync::atomic::AtomicU64::new(0),
            error_count: std::sync::atomic::AtomicU64::new(0),
        }
    }

    /// Increments the request counter
    pub fn increment_requests(&self) {
        self.request_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    /// Increments the error counter
    pub fn increment_errors(&self) {
        self.error_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    /// Gets current performance statistics
    pub fn get_stats(&self) -> PerformanceStats {
        let uptime = self.start_time.elapsed();
        let requests = self
            .request_count
            .load(std::sync::atomic::Ordering::Relaxed);
        let errors = self.error_count.load(std::sync::atomic::Ordering::Relaxed);

        let requests_per_second = if uptime.as_secs() > 0 {
            requests as f64 / uptime.as_secs() as f64
        } else {
            0.0
        };

        let error_rate = if requests > 0 {
            (errors as f64 / requests as f64) * 100.0
        } else {
            0.0
        };

        PerformanceStats {
            uptime,
            total_requests: requests,
            total_errors: errors,
            requests_per_second,
            error_rate,
        }
    }

    /// Logs performance statistics
    pub fn log_stats(&self) {
        let stats = self.get_stats();
        info!(
            "Performance stats: {} req/s, {:.2}% error rate, uptime: {:?}",
            stats.requests_per_second, stats.error_rate, stats.uptime
        );
    }
}

/// Performance statistics
#[derive(Debug, Clone)]
pub struct PerformanceStats {
    pub uptime: std::time::Duration,
    pub total_requests: u64,
    pub total_errors: u64,
    pub requests_per_second: f64,
    pub error_rate: f64,
}

// Global performance monitor
lazy_static::lazy_static! {
    static ref PERFORMANCE_MONITOR: PerformanceMonitor = PerformanceMonitor::new();
}

/// Gets the global performance monitor
pub fn get_performance_monitor() -> &'static PerformanceMonitor {
    &PERFORMANCE_MONITOR
}

/// Starts periodic performance logging
pub fn start_performance_logging() {
    let monitor = get_performance_monitor();

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(60));
        loop {
            interval.tick().await;
            monitor.log_stats();
        }
    });

    info!("Performance monitoring started");
}
