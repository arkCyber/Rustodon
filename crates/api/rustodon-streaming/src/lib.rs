//!
//! Rustodon Streaming Module
//!
//! This crate provides real-time streaming functionality for Rustodon using WebSockets.
//! It implements the streaming API similar to Mastodon's real-time timeline updates.
//!
//! # Examples
//!
//! ```rust
//! use rustodon_streaming::StreamingServer;
//!
//! #[tokio::main]
//! async fn main() {
//!     let server = StreamingServer::new("127.0.0.1:4000").await.unwrap();
//!     server.start().await.unwrap();
//! }
//! ```
//!
//! # Dependencies
//!
//! - `rustodon_core`: Core types and traits
//! - `rustodon_db`: Database operations
//! - `axum`: Web framework with WebSocket support
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use axum::{
    extract::{ws::WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use dashmap::DashMap;
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
    trace::TraceLayer,
};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// Streaming error type
#[derive(Debug, thiserror::Error)]
pub enum StreamingError {
    #[error("WebSocket error: {0}")]
    WebSocket(String),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Database error: {0}")]
    Database(String),
    #[error("Authentication error: {0}")]
    Authentication(String),
}

/// Stream types supported by the streaming API
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum StreamType {
    /// User's home timeline
    User,
    /// Public timeline
    Public,
    /// Local timeline
    Local,
    /// Hashtag timeline
    Hashtag(String),
    /// List timeline
    List(String),
    /// Direct messages
    Direct,
    /// User notifications
    Notifications,
}

/// Streaming message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event", content = "payload")]
pub enum StreamingMessage {
    /// Update event (new status)
    Update(serde_json::Value),
    /// Delete event (status deleted)
    Delete(String),
    /// Notification event
    Notification(serde_json::Value),
    /// Status update event
    StatusUpdate(serde_json::Value),
    /// Conversation event
    Conversation(serde_json::Value),
    /// Announcement event
    Announcement(serde_json::Value),
    /// Heartbeat event
    Heartbeat,
}

/// Client connection information
#[derive(Debug, Clone)]
pub struct ClientConnection {
    /// Unique client ID
    pub id: String,
    /// User ID if authenticated
    pub user_id: Option<i64>,
    /// Subscribed streams
    pub streams: Vec<StreamType>,
    /// Connection timestamp
    pub connected_at: chrono::DateTime<chrono::Utc>,
}

/// Streaming server implementation
#[derive(Clone)]
pub struct StreamingServer {
    /// Active client connections
    clients: Arc<DashMap<String, ClientConnection>>,
    /// Broadcast channels for different stream types
    channels: Arc<DashMap<StreamType, broadcast::Sender<StreamingMessage>>>,
    /// Server address
    address: String,
}

impl StreamingServer {
    /// Create a new streaming server
    ///
    /// # Arguments
    /// * `address` - Server address (e.g., "127.0.0.1:4000")
    ///
    /// # Returns
    /// Result with the streaming server or error
    pub async fn new(address: &str) -> Result<Self, StreamingError> {
        info!("Initializing streaming server on {}", address);

        let clients = Arc::new(DashMap::new());
        let channels = Arc::new(DashMap::new());

        // Initialize default channels
        let default_streams = vec![
            StreamType::Public,
            StreamType::Local,
            StreamType::User,
            StreamType::Direct,
            StreamType::Notifications,
        ];

        for stream_type in default_streams {
            let (tx, _) = broadcast::channel(1000);
            channels.insert(stream_type, tx);
        }

        Ok(Self {
            clients,
            channels,
            address: address.to_string(),
        })
    }

    /// Start the streaming server
    ///
    /// # Returns
    /// Result indicating success or failure
    pub async fn start(&self) -> Result<(), StreamingError> {
        info!("Starting streaming server on {}", self.address);

        let app = Router::new()
            .route("/api/v1/streaming", get(Self::streaming_handler))
            .route("/api/v1/streaming/health", get(Self::health_handler))
            .with_state(Arc::new(self.clone()))
            // Add middleware layers in correct order
            .layer(TraceLayer::new_for_http())
            .layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods(Any)
                    .allow_headers(Any)
                    .allow_credentials(true),
            )
            .layer(CompressionLayer::new())
            .layer(RequestBodyLimitLayer::new(1024 * 1024)); // 1MB limit for streaming

        let listener = tokio::net::TcpListener::bind(&self.address)
            .await
            .map_err(|e| {
                StreamingError::WebSocket(format!("Failed to bind to {}: {}", self.address, e))
            })?;

        info!("Streaming server listening on {}", self.address);

        axum::serve(listener, app)
            .await
            .map_err(|e| StreamingError::WebSocket(format!("Server error: {}", e)))?;

        Ok(())
    }

    /// WebSocket streaming handler
    async fn streaming_handler(
        ws: WebSocketUpgrade,
        axum::extract::State(state): axum::extract::State<Arc<Self>>,
    ) -> impl IntoResponse {
        info!("New WebSocket connection request");

        ws.on_upgrade(|socket| state.handle_socket(socket))
    }

    /// Health check handler
    async fn health_handler() -> axum::Json<serde_json::Value> {
        axum::Json(serde_json::json!({
            "status": "ok",
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))
    }

    /// Handle individual WebSocket connection
    async fn handle_socket(self: Arc<Self>, socket: WebSocket) {
        let client_id = Uuid::new_v4().to_string();
        info!("New WebSocket connection: {}", client_id);

        let (mut sender, mut receiver) = socket.split();
        let mut client_streams = Vec::new();
        let mut channel_receivers = Vec::new();

        // Send welcome message
        let welcome_msg = serde_json::json!({
            "event": "connected",
            "payload": {
                "client_id": client_id,
                "timestamp": chrono::Utc::now().to_rfc3339()
            }
        });

        if let Err(e) = sender
            .send(axum::extract::ws::Message::Text(welcome_msg.to_string()))
            .await
        {
            error!("Failed to send welcome message: {}", e);
            return;
        }

        // Handle incoming messages
        while let Some(msg) = receiver.next().await {
            match msg {
                Ok(axum::extract::ws::Message::Text(text)) => {
                    if let Err(e) = self
                        .handle_client_message(
                            &client_id,
                            &text,
                            &mut client_streams,
                            &mut channel_receivers,
                        )
                        .await
                    {
                        error!("Error handling client message: {}", e);
                        break;
                    }
                }
                Ok(axum::extract::ws::Message::Close(_)) => {
                    info!("Client {} disconnected", client_id);
                    break;
                }
                Err(e) => {
                    error!("WebSocket error for client {}: {}", client_id, e);
                    break;
                }
                _ => {}
            }
        }

        // Cleanup
        self.clients.remove(&client_id);
        info!("Client {} disconnected and cleaned up", client_id);
    }

    /// Handle client message
    async fn handle_client_message(
        &self,
        client_id: &str,
        message: &str,
        client_streams: &mut Vec<StreamType>,
        channel_receivers: &mut Vec<broadcast::Receiver<StreamingMessage>>,
    ) -> Result<(), StreamingError> {
        let parsed: serde_json::Value =
            serde_json::from_str(message).map_err(StreamingError::Serialization)?;

        if let Some(event) = parsed.get("event").and_then(|e| e.as_str()) {
            match event {
                "subscribe" => {
                    self.handle_subscribe(client_id, &parsed, client_streams, channel_receivers)
                        .await?;
                }
                "unsubscribe" => {
                    self.handle_unsubscribe(client_id, &parsed, client_streams)
                        .await?;
                }
                "ping" => {
                    self.handle_ping(client_id).await?;
                }
                _ => {
                    warn!("Unknown event type: {}", event);
                }
            }
        }

        Ok(())
    }

    /// Handle subscribe request
    async fn handle_subscribe(
        &self,
        client_id: &str,
        message: &serde_json::Value,
        client_streams: &mut Vec<StreamType>,
        channel_receivers: &mut Vec<broadcast::Receiver<StreamingMessage>>,
    ) -> Result<(), StreamingError> {
        let stream_type = self.parse_stream_type(message)?;

        // Subscribe to the stream
        if let Some(channel) = self.channels.get(&stream_type) {
            let receiver = channel.subscribe();
            channel_receivers.push(receiver);
            client_streams.push(stream_type.clone());

            debug!("Client {} subscribed to {:?}", client_id, stream_type);
        }

        Ok(())
    }

    /// Handle unsubscribe request
    async fn handle_unsubscribe(
        &self,
        client_id: &str,
        message: &serde_json::Value,
        client_streams: &mut Vec<StreamType>,
    ) -> Result<(), StreamingError> {
        let stream_type = self.parse_stream_type(message)?;

        // Remove from client streams
        client_streams.retain(|s| s != &stream_type);

        debug!("Client {} unsubscribed from {:?}", client_id, stream_type);
        Ok(())
    }

    /// Handle ping request
    async fn handle_ping(&self, client_id: &str) -> Result<(), StreamingError> {
        debug!("Client {} pinged", client_id);
        Ok(())
    }

    /// Parse stream type from message
    fn parse_stream_type(&self, message: &serde_json::Value) -> Result<StreamType, StreamingError> {
        let stream = message
            .get("stream")
            .and_then(|s| s.as_str())
            .ok_or_else(|| StreamingError::WebSocket("Missing stream parameter".to_string()))?;

        match stream {
            "user" => Ok(StreamType::User),
            "public" => Ok(StreamType::Public),
            "public:local" => Ok(StreamType::Local),
            "direct" => Ok(StreamType::Direct),
            "notifications" => Ok(StreamType::Notifications),
            s if s.starts_with("hashtag:") => {
                let hashtag = s.strip_prefix("hashtag:").unwrap_or("");
                Ok(StreamType::Hashtag(hashtag.to_string()))
            }
            s if s.starts_with("list:") => {
                let list_id = s.strip_prefix("list:").unwrap_or("");
                Ok(StreamType::List(list_id.to_string()))
            }
            _ => Err(StreamingError::WebSocket(format!(
                "Unknown stream type: {}",
                stream
            ))),
        }
    }

    /// Broadcast message to all subscribers of a stream
    pub async fn broadcast(
        &self,
        stream_type: &StreamType,
        message: StreamingMessage,
    ) -> Result<(), StreamingError> {
        if let Some(channel) = self.channels.get(stream_type) {
            if let Err(e) = channel.send(message) {
                warn!("Failed to broadcast message: {}", e);
            }
        }
        Ok(())
    }

    /// Get active client count
    pub fn active_clients(&self) -> usize {
        self.clients.len()
    }

    /// Get active stream count
    pub fn active_streams(&self) -> usize {
        self.channels.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_streaming_server_creation() {
        let server = StreamingServer::new("127.0.0.1:0").await;
        assert!(server.is_ok());
    }

    #[test]
    async fn test_stream_type_parsing() {
        let server = StreamingServer::new("127.0.0.1:0").await.unwrap();

        let message = serde_json::json!({
            "stream": "user"
        });

        let stream_type = server.parse_stream_type(&message).unwrap();
        assert!(matches!(stream_type, StreamType::User));
    }
}
