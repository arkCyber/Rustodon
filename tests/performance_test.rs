//!
//! Rustodon Performance Tests
//!
//! Performance benchmarks for critical operations in the Rustodon system.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use common::{TestDb, init_test_logging};
use rustodon_auth::{register_user, RegisterRequest};
use rustodon_search::{SearchIndex, SearchQuery};
use rustodon_workers::{Worker, ExampleJob};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, debug};
use std::time::Instant;

#[tokio::test]
async fn test_user_registration_performance() {
    init_test_logging();
    info!("Starting user registration performance test");

    let test_db = TestDb::new().await.expect("Failed to setup test database");
    let iterations = 100;
    let mut total_time = 0u128;

    for i in 0..iterations {
        let start = Instant::now();

        let register_req = RegisterRequest {
            username: format!("perfuser{}", i),
            email: format!("perf{}@example.com", i),
            password: "testpassword123".to_string(),
        };

        let result = register_user(register_req).await;
        assert!(result.is_ok(), "User registration should succeed");

        let duration = start.elapsed().as_millis();
        total_time += duration;

        if i % 10 == 0 {
            debug!("Registered user {} in {}ms", i, duration);
        }
    }

    let avg_time = total_time / iterations;
    info!("User registration performance: {}ms average over {} iterations", avg_time, iterations);

    test_db.cleanup().await.expect("Failed to cleanup test database");
}

#[tokio::test]
async fn test_search_performance() {
    init_test_logging();
    info!("Starting search performance test");

    let mut search_index = SearchIndex::new();

    // Add test documents
    for i in 0..1000 {
        search_index.add_document(
            &format!("doc_{}", i),
            &format!("Document {} with some test content for performance testing", i)
        ).await.expect("Failed to add document");
    }

    let iterations = 100;
    let mut total_time = 0u128;

    for i in 0..iterations {
        let start = Instant::now();

        let query = SearchQuery::new(format!("test{}", i % 10));
        let results = search_index.search(&query).await.expect("Search should succeed");

        let duration = start.elapsed().as_millis();
        total_time += duration;

        if i % 10 == 0 {
            debug!("Search {} found {} results in {}ms", i, results.len(), duration);
        }
    }

    let avg_time = total_time / iterations;
    info!("Search performance: {}ms average over {} iterations", avg_time, iterations);
}

#[tokio::test]
async fn test_worker_performance() {
    init_test_logging();
    info!("Starting worker performance test");

    let iterations = 100;
    let queue = Arc::new(Mutex::new(Vec::new()));

    // Add jobs to queue
    for _ in 0..iterations {
        queue.lock().await.push(Box::new(ExampleJob) as Box<dyn rustodon_workers::Job>);
    }

    let worker = Worker::new(queue.clone());
    let start = Instant::now();

    // Process jobs
    let worker_handle = tokio::spawn(async move {
        let _ = worker.start().await;
    });

    // Wait for jobs to be processed
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    worker_handle.abort();

    let total_time = start.elapsed().as_millis();
    let avg_time = total_time / iterations;

    info!("Worker performance: {}ms average per job over {} jobs", avg_time, iterations);
}
