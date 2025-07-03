//!
//! Rustodon Search Module
//!
//! This crate provides full-text search functionality for Rustodon, including indexing, query processing,
//! and result ranking. Supports search across posts, users, and hashtags.
//!
//! # Examples
//!
//! ```rust
//! use rustodon_search::{SearchIndex, SearchQuery, SearchResult};
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut index = SearchIndex::new();
//!     index.add_document("post_1", "Hello world!").await.unwrap();
//!     let query = SearchQuery::new("hello");
//!     let results = index.search(&query).await.unwrap();
//!     println!("Found {} results", results.len());
//! }
//! ```
//!
//! # Dependencies
//!
//! - `tokio`: Async runtime
//! - `tracing`: Structured logging
//! - `thiserror`: Error handling
//! - `serde`: Serialization
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use tracing::{debug, error, info};

/// Error type for search operations
#[derive(Error, Debug)]
pub enum SearchError {
    #[error("Index error: {0}")]
    Index(String),
    #[error("Query error: {0}")]
    Query(String),
    #[error("Document not found: {0}")]
    DocumentNotFound(String),
}

/// Search query with filters and options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    /// Search terms
    pub query: String,
    /// Document type filter (post, user, hashtag)
    pub document_type: Option<String>,
    /// Maximum number of results
    pub limit: Option<usize>,
    /// Offset for pagination
    pub offset: Option<usize>,
}

impl SearchQuery {
    /// Create a new search query
    pub fn new(query: impl Into<String>) -> Self {
        let query_str = query.into();
        info!("Creating search query: {}", query_str);
        Self {
            query: query_str,
            document_type: None,
            limit: Some(20),
            offset: Some(0),
        }
    }

    /// Set document type filter
    pub fn with_type(mut self, doc_type: impl Into<String>) -> Self {
        self.document_type = Some(doc_type.into());
        self
    }

    /// Set result limit
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Search result with relevance score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Document ID
    pub id: String,
    /// Document type
    pub document_type: String,
    /// Relevance score (0.0 to 1.0)
    pub score: f64,
    /// Document content snippet
    pub snippet: String,
}

/// In-memory search index for documents
pub struct SearchIndex {
    /// Document storage: ID -> (content, type)
    documents: HashMap<String, (String, String)>,
    /// Inverted index: term -> document IDs
    inverted_index: HashMap<String, Vec<String>>,
}

impl Default for SearchIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchIndex {
    /// Create a new search index
    pub fn new() -> Self {
        info!("Creating new search index");
        Self {
            documents: HashMap::new(),
            inverted_index: HashMap::new(),
        }
    }

    /// Add a document to the index
    pub async fn add_document(&mut self, id: &str, content: &str) -> Result<(), SearchError> {
        info!("Adding document to index: {}", id);

        // Store document
        self.documents
            .insert(id.to_string(), (content.to_string(), "post".to_string()));

        // Build inverted index
        let terms = self.tokenize(content);
        for term in terms {
            self.inverted_index
                .entry(term)
                .or_default()
                .push(id.to_string());
        }

        debug!("Document added successfully: {}", id);
        Ok(())
    }

    /// Search for documents matching the query
    pub async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, SearchError> {
        info!("Searching for: {}", query.query);

        let terms = self.tokenize(&query.query);
        let mut results = Vec::new();

        for term in terms {
            if let Some(doc_ids) = self.inverted_index.get(&term) {
                for doc_id in doc_ids {
                    if let Some((content, doc_type)) = self.documents.get(doc_id) {
                        // Apply document type filter
                        if let Some(ref filter_type) = query.document_type {
                            if doc_type != filter_type {
                                continue;
                            }
                        }

                        let score = self.calculate_score(&query.query, content);
                        let snippet = self.generate_snippet(content, &query.query);

                        results.push(SearchResult {
                            id: doc_id.clone(),
                            document_type: doc_type.clone(),
                            score,
                            snippet,
                        });
                    }
                }
            }
        }

        // Sort by score and apply pagination
        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let offset = query.offset.unwrap_or(0);
        let limit = query.limit.unwrap_or(20);

        Ok(results.into_iter().skip(offset).take(limit).collect())
    }

    /// Tokenize text into search terms
    fn tokenize(&self, text: &str) -> Vec<String> {
        text.to_lowercase()
            .split_whitespace()
            .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()))
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    }

    /// Calculate relevance score for a document
    fn calculate_score(&self, query: &str, content: &str) -> f64 {
        let query_terms: Vec<String> = query.split_whitespace().map(|s| s.to_string()).collect();
        let content_lower = content.to_lowercase();
        let query_len = query_terms.len();

        let mut score = 0.0;
        for term in &query_terms {
            if content_lower.contains(&term.to_lowercase()) {
                score += 1.0;
            }
        }

        score / query_len as f64
    }

    /// Generate a snippet highlighting search terms
    fn generate_snippet(&self, content: &str, _query: &str) -> String {
        let max_length = 150;
        if content.len() <= max_length {
            return content.to_string();
        }

        let start = content.len() / 2 - max_length / 2;
        let end = start + max_length;
        format!("...{}...", &content[start..end])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_and_search_document() {
        let mut index = SearchIndex::new();
        index
            .add_document("post_1", "Hello world! This is a test post.")
            .await
            .unwrap();

        let query = SearchQuery::new("hello");
        let results = index.search(&query).await.unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "post_1");
        assert!(results[0].score > 0.0);
    }

    #[tokio::test]
    async fn test_search_with_type_filter() {
        let mut index = SearchIndex::new();
        index.add_document("post_1", "Hello world!").await.unwrap();

        let query = SearchQuery::new("hello").with_type("post");
        let results = index.search(&query).await.unwrap();

        assert_eq!(results.len(), 1);
    }

    #[tokio::test]
    async fn test_search_no_results() {
        let index = SearchIndex::new();
        let query = SearchQuery::new("nonexistent");
        let results = index.search(&query).await.unwrap();

        assert_eq!(results.len(), 0);
    }
}
