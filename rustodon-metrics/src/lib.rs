//!
//! Rustodon Metrics and Monitoring
//!
//! This module provides comprehensive metrics collection and monitoring for the Rustodon server,
//! supporting Prometheus format metrics, custom counters, gauges, and histograms.
//!
//! # Examples
//!
//! ```rust
//! use rustodon_metrics::{Metrics, Counter, Gauge, Histogram};
//!
//! let metrics = Metrics::new();
//! let request_counter = metrics.counter("http_requests_total", "Total HTTP requests");
//! let active_connections = metrics.gauge("active_connections", "Number of active connections");
//! let request_duration = metrics.histogram("request_duration_seconds", "Request duration");
//!
//! request_counter.increment();
//! active_connections.set(42.0);
//! request_duration.observe(0.5);
//! ```
//!
//! # Dependencies
//!
//! - `prometheus`: Prometheus metrics library
//! - `serde`: Serialization for metrics export
//! - `thiserror`: Error handling
//! - `tracing`: Logging
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use prometheus::{Counter, Gauge, Histogram, HistogramOpts, IntCounter, IntGauge, Opts, Registry};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tracing::{debug, error, info};

/// Metrics error type
#[derive(Error, Debug)]
pub enum MetricsError {
    #[error("Failed to create metric: {0}")]
    MetricCreationError(String),
    #[error("Failed to register metric: {0}")]
    RegistrationError(String),
    #[error("Failed to serialize metrics: {0}")]
    SerializationError(String),
    #[error("Invalid metric name: {0}")]
    InvalidMetricName(String),
    #[error("Invalid metric value: {0}")]
    InvalidMetricValue(String),
}

/// Metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// Whether metrics collection is enabled
    pub enabled: bool,
    /// Metrics endpoint path
    pub endpoint: String,
    /// Metrics port
    pub port: u16,
    /// Whether to enable default metrics
    pub enable_default_metrics: bool,
    /// Custom labels to add to all metrics
    pub default_labels: HashMap<String, String>,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        let mut default_labels = HashMap::new();
        default_labels.insert("service".to_string(), "rustodon".to_string());
        default_labels.insert("version".to_string(), env!("CARGO_PKG_VERSION").to_string());

        Self {
            enabled: true,
            endpoint: "/metrics".to_string(),
            port: 9090,
            enable_default_metrics: true,
            default_labels,
        }
    }
}

/// Main metrics manager
#[derive(Clone)]
pub struct Metrics {
    registry: Arc<Registry>,
    config: MetricsConfig,
    counters: Arc<tokio::sync::RwLock<HashMap<String, Counter>>>,
    gauges: Arc<tokio::sync::RwLock<HashMap<String, Gauge>>>,
    histograms: Arc<tokio::sync::RwLock<HashMap<String, Histogram>>>,
    int_counters: Arc<tokio::sync::RwLock<HashMap<String, IntCounter>>>,
    int_gauges: Arc<tokio::sync::RwLock<HashMap<String, IntGauge>>>,
}

impl Metrics {
    /// Create a new metrics instance
    pub fn new() -> Self {
        Self::with_config(MetricsConfig::default())
    }

    /// Create a new metrics instance with custom configuration
    pub fn with_config(config: MetricsConfig) -> Self {
        let registry = Arc::new(Registry::new());

        if config.enable_default_metrics {
            Self::register_default_metrics(&registry);
        }

        Self {
            registry,
            config,
            counters: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            gauges: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            histograms: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            int_counters: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            int_gauges: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }

    /// Register default metrics
    fn register_default_metrics(registry: &Registry) {
        // HTTP request metrics
        let http_requests_total =
            Counter::new("http_requests_total", "Total number of HTTP requests").unwrap();
        registry
            .register(Box::new(http_requests_total.clone()))
            .unwrap();

        let http_request_duration_seconds = Histogram::with_opts(HistogramOpts::new(
            "http_request_duration_seconds",
            "HTTP request duration in seconds",
        ))
        .unwrap();
        registry
            .register(Box::new(http_request_duration_seconds.clone()))
            .unwrap();

        // Database metrics
        let db_connections_active = Gauge::new(
            "db_connections_active",
            "Number of active database connections",
        )
        .unwrap();
        registry
            .register(Box::new(db_connections_active.clone()))
            .unwrap();

        let db_queries_total =
            Counter::new("db_queries_total", "Total number of database queries").unwrap();
        registry
            .register(Box::new(db_queries_total.clone()))
            .unwrap();

        // Application metrics
        let app_users_total =
            IntGauge::new("app_users_total", "Total number of registered users").unwrap();
        registry
            .register(Box::new(app_users_total.clone()))
            .unwrap();

        let app_statuses_total =
            IntGauge::new("app_statuses_total", "Total number of statuses").unwrap();
        registry
            .register(Box::new(app_statuses_total.clone()))
            .unwrap();

        info!("Default metrics registered");
    }

    /// Create a new counter metric
    pub async fn counter(&self, name: &str, help: &str) -> Result<Counter, MetricsError> {
        let mut counters = self.counters.write().await;

        if let Some(counter) = counters.get(name) {
            return Ok(counter.clone());
        }

        let opts = Opts::new(name, help);
        let counter = Counter::with_opts(opts)
            .map_err(|e| MetricsError::MetricCreationError(e.to_string()))?;

        self.registry
            .register(Box::new(counter.clone()))
            .map_err(|e| MetricsError::RegistrationError(e.to_string()))?;

        counters.insert(name.to_string(), counter.clone());
        debug!("Created counter metric: {}", name);

        Ok(counter)
    }

    /// Create a new gauge metric
    pub async fn gauge(&self, name: &str, help: &str) -> Result<Gauge, MetricsError> {
        let mut gauges = self.gauges.write().await;

        if let Some(gauge) = gauges.get(name) {
            return Ok(gauge.clone());
        }

        let opts = Opts::new(name, help);
        let gauge =
            Gauge::with_opts(opts).map_err(|e| MetricsError::MetricCreationError(e.to_string()))?;

        self.registry
            .register(Box::new(gauge.clone()))
            .map_err(|e| MetricsError::RegistrationError(e.to_string()))?;

        gauges.insert(name.to_string(), gauge.clone());
        debug!("Created gauge metric: {}", name);

        Ok(gauge)
    }

    /// Create a new histogram metric
    pub async fn histogram(&self, name: &str, help: &str) -> Result<Histogram, MetricsError> {
        let mut histograms = self.histograms.write().await;

        if let Some(histogram) = histograms.get(name) {
            return Ok(histogram.clone());
        }

        let opts = HistogramOpts::new(name, help);
        let histogram = Histogram::with_opts(opts)
            .map_err(|e| MetricsError::MetricCreationError(e.to_string()))?;

        self.registry
            .register(Box::new(histogram.clone()))
            .map_err(|e| MetricsError::RegistrationError(e.to_string()))?;

        histograms.insert(name.to_string(), histogram.clone());
        debug!("Created histogram metric: {}", name);

        Ok(histogram)
    }

    /// Create a new integer counter metric
    pub async fn int_counter(&self, name: &str, help: &str) -> Result<IntCounter, MetricsError> {
        let mut int_counters = self.int_counters.write().await;

        if let Some(counter) = int_counters.get(name) {
            return Ok(counter.clone());
        }

        let opts = Opts::new(name, help);
        let counter = IntCounter::with_opts(opts)
            .map_err(|e| MetricsError::MetricCreationError(e.to_string()))?;

        self.registry
            .register(Box::new(counter.clone()))
            .map_err(|e| MetricsError::RegistrationError(e.to_string()))?;

        int_counters.insert(name.to_string(), counter.clone());
        debug!("Created int counter metric: {}", name);

        Ok(counter)
    }

    /// Create a new integer gauge metric
    pub async fn int_gauge(&self, name: &str, help: &str) -> Result<IntGauge, MetricsError> {
        let mut int_gauges = self.int_gauges.write().await;

        if let Some(gauge) = int_gauges.get(name) {
            return Ok(gauge.clone());
        }

        let opts = Opts::new(name, help);
        let gauge = IntGauge::with_opts(opts)
            .map_err(|e| MetricsError::MetricCreationError(e.to_string()))?;

        self.registry
            .register(Box::new(gauge.clone()))
            .map_err(|e| MetricsError::RegistrationError(e.to_string()))?;

        int_gauges.insert(name.to_string(), gauge.clone());
        debug!("Created int gauge metric: {}", name);

        Ok(gauge)
    }

    /// Get metrics in Prometheus format
    pub fn gather(&self) -> Result<String, MetricsError> {
        use prometheus::Encoder;
        let encoder = prometheus::TextEncoder::new();
        let mut buffer = Vec::new();

        encoder
            .encode(&self.registry.gather(), &mut buffer)
            .map_err(|e| MetricsError::SerializationError(e.to_string()))?;

        String::from_utf8(buffer).map_err(|e| MetricsError::SerializationError(e.to_string()))
    }

    /// Record HTTP request metrics
    pub async fn record_http_request(
        &self,
        method: &str,
        path: &str,
        status_code: u16,
        duration: f64,
    ) {
        if !self.config.enabled {
            return;
        }

        // Increment request counter
        if let Ok(counter) = self
            .counter("http_requests_total", "Total HTTP requests")
            .await
        {
            counter.inc();
        }

        // Record request duration
        if let Ok(histogram) = self
            .histogram("http_request_duration_seconds", "HTTP request duration")
            .await
        {
            histogram.observe(duration);
        }

        // Record status code distribution
        let _status_label = if status_code < 400 {
            "success"
        } else {
            "error"
        };
        if let Ok(counter) = self
            .counter("http_requests_by_status", "HTTP requests by status")
            .await
        {
            counter.inc();
        }

        debug!(
            "Recorded HTTP request metrics: {} {} {} {:.3}s",
            method, path, status_code, duration
        );
    }

    /// Record database operation metrics
    pub async fn record_db_operation(
        &self,
        operation: &str,
        table: &str,
        duration: f64,
        success: bool,
    ) {
        if !self.config.enabled {
            return;
        }

        // Increment query counter
        if let Ok(counter) = self
            .counter("db_queries_total", "Total database queries")
            .await
        {
            counter.inc();
        }

        // Record query duration
        if let Ok(histogram) = self
            .histogram("db_query_duration_seconds", "Database query duration")
            .await
        {
            histogram.observe(duration);
        }

        // Record success/failure
        let result_label = if success { "success" } else { "error" };
        if let Ok(counter) = self
            .counter("db_operations_by_result", "Database operations by result")
            .await
        {
            counter.inc();
        }

        debug!(
            "Recorded DB operation metrics: {} {} {:.3}s {}",
            operation, table, duration, result_label
        );
    }

    /// Update application metrics
    pub async fn update_app_metrics(&self, users_count: i64, statuses_count: i64) {
        if !self.config.enabled {
            return;
        }

        if let Ok(gauge) = self
            .int_gauge("app_users_total", "Total number of registered users")
            .await
        {
            gauge.set(users_count);
        }

        if let Ok(gauge) = self
            .int_gauge("app_statuses_total", "Total number of statuses")
            .await
        {
            gauge.set(statuses_count);
        }

        debug!(
            "Updated app metrics: users={}, statuses={}",
            users_count, statuses_count
        );
    }

    /// Get metrics configuration
    pub fn config(&self) -> &MetricsConfig {
        &self.config
    }

    /// Check if metrics are enabled
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Metrics middleware for HTTP requests
pub struct MetricsMiddleware {
    metrics: Metrics,
}

impl MetricsMiddleware {
    /// Create a new metrics middleware
    pub fn new(metrics: Metrics) -> Self {
        Self { metrics }
    }

    /// Record request metrics
    pub async fn record_request(&self, method: &str, path: &str, status_code: u16, duration: f64) {
        self.metrics
            .record_http_request(method, path, status_code, duration)
            .await;
    }
}

/// Metrics collector for periodic updates
pub struct MetricsCollector {
    metrics: Metrics,
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new(metrics: Metrics) -> Self {
        Self { metrics }
    }

    /// Start collecting metrics periodically
    pub async fn start_collection(&self) {
        info!("Starting metrics collection");

        let metrics = self.metrics.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60));

            loop {
                interval.tick().await;

                // Update application metrics
                // TODO: Get actual counts from database
                metrics.update_app_metrics(100, 1000).await;

                debug!("Updated periodic metrics");
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_metrics_creation() {
        let metrics = Metrics::new();
        assert!(metrics.is_enabled());

        let counter = metrics
            .counter("test_counter", "Test counter")
            .await
            .unwrap();
        counter.inc();
        assert_eq!(counter.get(), 1.0);
    }

    #[tokio::test]
    async fn test_metrics_gathering() {
        let metrics = Metrics::new();
        let counter = metrics.counter("test_gather", "Test gather").await.unwrap();
        counter.inc();

        let output = metrics.gather().unwrap();
        assert!(output.contains("test_gather"));
        assert!(output.contains("1"));
    }

    #[tokio::test]
    async fn test_http_request_recording() {
        let metrics = Metrics::new();
        metrics
            .record_http_request("GET", "/api/v1/statuses", 200, 0.1)
            .await;

        let output = metrics.gather().unwrap();
        assert!(output.contains("http_requests_total"));
    }

    #[tokio::test]
    async fn test_db_operation_recording() {
        let metrics = Metrics::new();
        metrics
            .record_db_operation("SELECT", "users", 0.05, true)
            .await;

        let output = metrics.gather().unwrap();
        assert!(output.contains("db_queries_total"));
    }
}
