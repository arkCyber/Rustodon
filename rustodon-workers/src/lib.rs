//!
//! Rustodon Workers Module
//!
//! This crate provides background job processing for Rustodon, including task queue management and worker execution.
//! Uses async/await and the `tokio` runtime for concurrency.
//!
//! # Examples
//!
//! ```rust
//! use rustodon_workers::{Job, Worker, WorkerError};
//! use std::sync::Arc;
//! use tokio::sync::Mutex;
//!
//! #[tokio::main]
//! async fn main() {
//!     let queue = Arc::new(Mutex::new(vec![]));
//!     let worker = Worker::new(queue.clone());
//!     // worker.start().await.unwrap(); // This would run forever
//! }
//! ```
//!
//! # Dependencies
//!
//! - `tokio`: Async runtime
//! - `tracing`: Structured logging
//! - `thiserror`: Error handling
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::Mutex;
use tracing::{debug, error, info};

/// Error type for worker operations
#[derive(Error, Debug)]
pub enum WorkerError {
    #[error("Job execution error: {0}")]
    Job(String),
    #[error("Queue error: {0}")]
    Queue(String),
}

/// Trait for background jobs
pub trait Job: Send + Sync + 'static {
    /// Execute the job asynchronously
    fn name(&self) -> &'static str;
    fn execute(&self) -> Pin<Box<dyn Future<Output = Result<(), WorkerError>> + Send>>;
}

/// Example job implementation
pub struct ExampleJob;

impl Job for ExampleJob {
    fn name(&self) -> &'static str {
        "ExampleJob"
    }
    fn execute(&self) -> Pin<Box<dyn Future<Output = Result<(), WorkerError>> + Send>> {
        Box::pin(async move {
            debug!("Executing ExampleJob");
            Ok(())
        })
    }
}

/// Worker struct that processes jobs from a queue
pub struct Worker {
    queue: Arc<Mutex<Vec<Box<dyn Job>>>>,
}

impl Worker {
    /// Create a new worker with a shared job queue
    pub fn new(queue: Arc<Mutex<Vec<Box<dyn Job>>>>) -> Self {
        info!("Creating new Worker");
        Self { queue }
    }

    /// Start processing jobs in the queue
    pub async fn start(&self) -> Result<(), WorkerError> {
        info!("Worker started");
        loop {
            let job_opt = {
                let mut queue = self.queue.lock().await;
                queue.pop()
            };
            match job_opt {
                Some(job) => {
                    info!("Processing job: {}", job.name());
                    if let Err(e) = job.execute().await {
                        error!("Job failed: {}", e);
                    } else {
                        info!("Job completed: {}", job.name());
                    }
                }
                None => {
                    debug!("No jobs in queue, worker sleeping");
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    struct TestJob;
    impl Job for TestJob {
        fn name(&self) -> &'static str {
            "TestJob"
        }
        fn execute(&self) -> Pin<Box<dyn Future<Output = Result<(), WorkerError>> + Send>> {
            Box::pin(async move { Ok(()) })
        }
    }

    #[tokio::test]
    async fn test_worker_processes_job() {
        let queue = Arc::new(Mutex::new(vec![Box::new(TestJob) as Box<dyn Job>]));
        let worker = Worker::new(queue.clone());
        // Run the worker for a short time, then break
        tokio::spawn(async move {
            let _ = worker.start().await;
        });
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        let queue = queue.lock().await;
        assert!(queue.is_empty());
    }
}
