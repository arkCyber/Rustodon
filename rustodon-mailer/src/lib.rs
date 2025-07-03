//!
//! Rustodon Mailer Module
//!
//! This crate provides async email sending functionality for Rustodon, including trait-based abstraction,
//! error handling, logging, and mock/test support. Designed for easy extension to SMTP, Sendmail, etc.
//!
//! # Examples
//!
//! ```rust
//! use rustodon_mailer::{AsyncMailer, Email, MailerError, MockMailer};
//!
//! #[tokio::main]
//! async fn main() {
//!     let mailer = MockMailer::default();
//!     let email = Email {
//!         to: "user@example.com".to_string(),
//!         subject: "Test".to_string(),
//!         body: "Hello!".to_string(),
//!     };
//!     mailer.send(email).await.unwrap();
//! }
//! ```
//!
//! # Dependencies
//!
//! - `tokio`: Async runtime
//! - `tracing`: Structured logging
//! - `thiserror`: Error handling
//! - `serde`: Serialization
//! - `async-trait`: Async trait support
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{error, info};

/// Error type for mailer operations
#[derive(Error, Debug)]
pub enum MailerError {
    #[error("Send error: {0}")]
    Send(String),
    #[error("Config error: {0}")]
    Config(String),
}

/// Email message struct
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Email {
    /// Recipient email address
    pub to: String,
    /// Email subject
    pub subject: String,
    /// Email body (plain text)
    pub body: String,
}

/// Async mailer trait
#[async_trait]
pub trait AsyncMailer: Send + Sync {
    /// Send an email asynchronously
    async fn send(&self, email: Email) -> Result<(), MailerError>;
}

/// Mock mailer for testing and development
#[derive(Default, Debug, Clone)]
pub struct MockMailer;

#[async_trait]
impl AsyncMailer for MockMailer {
    async fn send(&self, email: Email) -> Result<(), MailerError> {
        info!(
            "MockMailer: sending email to {} with subject '{}': {}",
            email.to, email.subject, email.body
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_mailer_send() {
        let mailer = MockMailer;
        let email = Email {
            to: "test@example.com".to_string(),
            subject: "Hello".to_string(),
            body: "Test body".to_string(),
        };
        let result = mailer.send(email).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_email_struct() {
        let email = Email {
            to: "a@b.com".to_string(),
            subject: "S".to_string(),
            body: "B".to_string(),
        };
        assert_eq!(email.to, "a@b.com");
        assert_eq!(email.subject, "S");
        assert_eq!(email.body, "B");
    }
}
