//!
//! Rustodon Integration Tests
//!
//! End-to-end tests for the complete Rustodon system, including API, auth, database, workers, and mailer.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

mod common;

use common::{TestDb, TestUser, init_test_logging};
use rustodon_auth::{register_user, login_user, RegisterRequest, LoginRequest};
use rustodon_workers::{Worker, ExampleJob};
use rustodon_mailer::{MockMailer, Email, AsyncMailer};
use rustodon_search::{SearchIndex, SearchQuery};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, debug};

#[tokio::test]
async fn test_full_user_flow() {
    init_test_logging();
    info!("Starting full user flow integration test");

    // Setup test database
    let test_db = TestDb::new().await.expect("Failed to setup test database");
    debug!("Test database setup complete");

    // Test user registration
    let test_user = TestUser::default();
    let register_req = RegisterRequest {
        username: test_user.username.clone(),
        email: test_user.email.clone(),
        password: test_user.password.clone(),
    };

    let register_result = register_user(register_req).await;
    assert!(register_result.is_ok(), "User registration should succeed");
    debug!("User registration successful");

    // Test user login
    let login_req = LoginRequest {
        username_or_email: test_user.username.clone(),
        password: test_user.password.clone(),
    };

    let login_result = login_user(login_req).await;
    assert!(login_result.is_ok(), "User login should succeed");
    debug!("User login successful");

    // Test worker processing
    let queue = Arc::new(Mutex::new(vec![Box::new(ExampleJob) as Box<dyn rustodon_workers::Job>]));
    let worker = Worker::new(queue.clone());

    // Process one job
    let worker_handle = tokio::spawn(async move {
        let _ = worker.start().await;
    });

    // Give worker time to process
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    worker_handle.abort();
    debug!("Worker processing test complete");

    // Test mailer
    let mailer = MockMailer::default();
    let email = Email {
        to: test_user.email.clone(),
        subject: "Integration Test".to_string(),
        body: "This is a test email from integration tests.".to_string(),
    };

    let mail_result = mailer.send(email).await;
    assert!(mail_result.is_ok(), "Email sending should succeed");
    debug!("Email sending test complete");

    // Test search functionality
    let mut search_index = SearchIndex::new();
    search_index.add_document("post_1", "Hello world! This is a test post.").await.expect("Failed to add document to search index");

    let query = SearchQuery::new("hello");
    let search_results = search_index.search(&query).await.expect("Search should succeed");
    assert_eq!(search_results.len(), 1, "Should find one search result");
    debug!("Search functionality test complete");

    // Cleanup
    test_db.cleanup().await.expect("Failed to cleanup test database");
    info!("Full user flow integration test completed successfully");
}

#[tokio::test]
async fn test_api_endpoints() {
    init_test_logging();
    info!("Starting API endpoints integration test");

    // Test health endpoint
    let app = rustodon_api::create_router();
    let response = axum_test::TestServer::new(app).unwrap()
        .get("/api/v1/health")
        .await;

    assert_eq!(response.status_code(), 200);
    debug!("Health endpoint test complete");

    info!("API endpoints integration test completed successfully");
}

#[tokio::test]
async fn test_error_handling() {
    init_test_logging();
    info!("Starting error handling integration test");

    // Test invalid login
    let invalid_login_req = LoginRequest {
        username_or_email: "nonexistent".to_string(),
        password: "wrongpassword".to_string(),
    };

    let login_result = login_user(invalid_login_req).await;
    assert!(login_result.is_err(), "Invalid login should fail");
    debug!("Invalid login error handling test complete");

    // Test search with no results
    let search_index = SearchIndex::new();
    let query = SearchQuery::new("nonexistent");
    let search_results = search_index.search(&query).await.expect("Search should succeed even with no results");
    assert_eq!(search_results.len(), 0, "Should find no search results");
    debug!("Empty search results test complete");

    info!("Error handling integration test completed successfully");
}
