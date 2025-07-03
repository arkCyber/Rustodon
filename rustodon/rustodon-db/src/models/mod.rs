//! Database models for Rustodon
//!
//! This module contains all database models and their operations.
//! Each model is organized in its own submodule for better maintainability.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

pub mod block;
pub mod domain_block;
pub mod favourite;
pub mod filter;
pub mod follow;
pub mod list;
pub mod mute;
pub mod oauth_access_token;
pub mod oauth_application;
pub mod reblog;
pub mod status;
pub mod user;

// Re-export main models
pub use block::Block;
pub use domain_block::DomainBlock;
pub use favourite::Favourite;
pub use filter::Filter;
pub use follow::Follow;
pub use list::{List, ListAccount};
pub use mute::Mute;
pub use oauth_access_token::OAuthAccessToken;
pub use oauth_application::OAuthApplication;
pub use reblog::Reblog;
pub use status::Status;
pub use user::{User, UserStatus};
