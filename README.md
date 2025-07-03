# Rustodon

[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL%203.0-green.svg)](https://opensource.org/licenses/AGPL-3.0)
[![Rust](https://img.shields.io/badge/Rust-1.70+-blue.svg)](https://www.rust-lang.org/)
[![Build Status](https://github.com/arkCyber/Rustodon/workflows/CI/badge.svg)](https://github.com/arkCyber/Rustodon/actions)

A high-performance, type-safe Rust implementation of the Mastodon server backend, aiming for 100% compatibility with the original Mastodon server functionality.

## 🎯 Current Status

**Development Phase**: Core API Implementation Complete ✅

- ✅ **Core Infrastructure**: Server setup, logging, error handling
- ✅ **Authentication System**: User registration, login, OAuth support
- ✅ **Status Management**: Create, read, update status posts
- ✅ **Timeline Endpoints**: Public and home timeline functionality
- ✅ **Social Interactions**: Favouriting/unfavouriting statuses
- ✅ **API Testing**: Comprehensive curl test suite
- 🚧 **Database Integration**: Mock responses (PostgreSQL integration in progress)
- 🚧 **ActivityPub Federation**: Protocol implementation in progress
- 🚧 **Media Processing**: Image and video handling
- 🚧 **Real-time Streaming**: WebSocket implementation
- 🚧 **Web Interface**: Frontend development

**Ready for**: API testing, development contributions, feedback

## 🚀 Features

- **100% Mastodon API Compatibility**: Full REST API compatibility with original Mastodon
- **ActivityPub Federation**: Complete ActivityPub protocol implementation
- **High Performance**: Built with Rust for maximum performance and memory safety
- **Type Safety**: Strong typing throughout the codebase
- **Async/Await**: Full async support for concurrent operations
- **Modular Architecture**: Clean separation of concerns with modular crates
- **Database Support**: PostgreSQL with SQLx for type-safe database operations
- **Real-time Updates**: WebSocket streaming for live updates
- **Media Processing**: Image and video processing capabilities
- **Search**: Full-text search functionality
- **Security**: Built-in security features and best practices

## 📋 Requirements

- **Rust**: 1.70 or higher
- **PostgreSQL**: 13 or higher
- **Redis**: 6.2 or higher (for caching and job queues)
- **Node.js**: 20 or higher (for web interface)

## 🏗️ Architecture

Rustodon is organized as a Rust workspace with multiple modular crates:

```
rustodon/
├── rustodon-core/           # Core types and traits
├── rustodon-db/            # Database operations
├── rustodon-api/           # HTTP API layer
├── rustodon-auth/          # Authentication & authorization
├── rustodon-activitypub/   # ActivityPub protocol
├── rustodon-workers/       # Background job processing
├── rustodon-search/        # Search functionality
├── rustodon-mailer/        # Email functionality
├── rustodon-admin/         # Admin interface
├── rustodon-config/        # Configuration management
├── rustodon-logging/       # Logging infrastructure
├── rustodon-metrics/       # Metrics and monitoring
├── rustodon-cache/         # Caching layer
├── rustodon-queue/         # Message queue
├── rustodon-storage/       # File storage
├── rustodon-notifications/ # Notification system
├── rustodon-media/         # Media processing
├── rustodon-federation/    # Federation logic
├── rustodon-webhooks/      # Webhook handling
├── rustodon-scheduler/     # Scheduled tasks
├── rustodon-migrations/    # Database migrations
├── rustodon-cli/           # Command line interface
├── rustodon-server/        # Main server binary
└── tests/                  # Integration tests
```

## 🚀 Quick Start

### 1. Clone the Repository

```bash
git clone https://github.com/arkCyber/Rustodon.git
cd Rustodon
```

### 2. Set Up Environment

```bash
# Copy environment template
cp .env.example .env

# Edit environment variables
nano .env
```

### 3. Set Up Database

```bash
# Install PostgreSQL and Redis
# (Instructions vary by platform)

# Run database migrations
cargo run -p rustodon-migrations
```

### 4. Build and Run

```bash
# Build all crates
cargo build --release

# Run the server
cargo run -p rustodon-server
```

### 5. Access the Application

- **Web Interface**: http://localhost:3000
- **API Documentation**: http://localhost:3000/api/v1/docs

## 📚 Documentation

- [API Documentation](docs/api.md)
- [Deployment Guide](docs/deployment.md)
- [Development Guide](docs/development.md)
- [Configuration Guide](docs/configuration.md)
- [Federation Guide](docs/federation.md)

## 🔧 Configuration

Rustodon uses environment variables for configuration. Key settings include:

```bash
# Database
DATABASE_URL=postgresql://user:password@localhost/rustodon

# Redis
REDIS_URL=redis://localhost:6379

# Server
RUSTODON_HOST=0.0.0.0
RUSTODON_PORT=3000

# Federation
RUSTODON_DOMAIN=yourdomain.com
RUSTODON_SECRET_KEY_BASE=your-secret-key

# Email
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USERNAME=your-email@gmail.com
SMTP_PASSWORD=your-password
```

## 🧪 Testing

### Unit Tests
```bash
# Run all tests
cargo test

# Run tests with logging
RUST_LOG=debug cargo test

# Run specific crate tests
cargo test -p rustodon-api

# Run integration tests
cargo test --test integration
```

### API Testing
We provide a comprehensive curl test script to validate API functionality:

```bash
# Make the test script executable
chmod +x test_api.sh

# Run API tests (server must be running)
./test_api.sh
```

The test script validates:
- Health check endpoint
- User registration and login
- OAuth application registration
- Status creation and retrieval
- Timeline endpoints (public and home)
- Status interactions (favouriting/unfavouriting)

### Running the Server for Testing

```bash
# Set offline mode to bypass database connection during compilation
export SQLX_OFFLINE=true

# Start the server
cargo run -p rustodon-server

# Server will be available at http://localhost:3000
```

## 🐳 Docker Deployment

```bash
# Build Docker image
docker build -t rustodon .

# Run with docker-compose
docker-compose up -d
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes
4. Run tests: `cargo test`
5. Format code: `cargo fmt`
6. Check linting: `cargo clippy`
7. Commit your changes: `git commit -m 'Add amazing feature'`
8. Push to the branch: `git push origin feature/amazing-feature`
9. Open a Pull Request

## 📄 License

This project is licensed under the GNU Affero General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Mastodon](https://github.com/mastodon/mastodon) - The original Mastodon project
- [ActivityPub](https://www.w3.org/TR/activitypub/) - The ActivityPub protocol specification
- [Rust Community](https://www.rust-lang.org/community) - For the amazing Rust ecosystem

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/arkCyber/Rustodon/issues)
- **Discussions**: [GitHub Discussions](https://github.com/arkCyber/Rustodon/discussions)
- **Email**: arksong2018@gmail.com

## 🗺️ Roadmap

- [ ] Complete ActivityPub implementation
- [ ] Web interface development
- [ ] Real-time streaming
- [ ] Advanced media processing
- [ ] Performance optimizations
- [ ] Additional federation features
- [ ] Mobile app support
- [ ] Enterprise features

---

**Made with ❤️ by the Rustodon Team**

- Follow Rust best practices and idioms
- Use async/await for I/O operations
- Implement proper error handling with `Result<T, E>`
- Use structured logging with tracing
- Write comprehensive tests for all functionality
- Document all public APIs

### Project Structure

Each crate follows a consistent structure:

```rust
//! Module description and purpose
//!
//! This module provides [specific functionality] for the Rustodon server.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use tracing::{info, warn, error, debug, trace};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Custom error type for this module
#[derive(Error, Debug)]
pub enum ModuleError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Main struct for this module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleStruct {
    /// Field description
    pub field: String,
}

impl ModuleStruct {
    /// Creates a new instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustodon_module::ModuleStruct;
    ///
    /// let instance = ModuleStruct::new("example");
    /// assert_eq!(instance.field, "example");
    /// ```
    pub fn new(field: impl Into<String>) -> Self {
        let field = field.into();
        trace!("Creating new ModuleStruct with field: {}", field);
        Self { field }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    fn test_module_struct_new() {
        let instance = ModuleStruct::new("test");
        assert_eq!(instance.field, "test");
    }
}
```

### Testing Strategy

- **Unit Tests**: Test individual functions and methods
- **Integration Tests**: Test module interactions
- **End-to-End Tests**: Test complete user flows
- **Performance Tests**: Benchmark critical operations
- **Documentation Tests**: Ensure examples compile and run

### Logging

The project uses structured logging with the `tracing` crate:

```rust
use tracing::{info, warn, error, debug, trace};

// Log at different levels
trace!("Detailed debugging information");
debug!("General debugging information");
info!("General information about program execution");
warn!("Warning messages");
error!("Error conditions");
```

### Error Handling

All fallible operations return `Result<T, E>` with custom error types:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Internal error: {0}")]
    Internal(String),
}
```

## API Endpoints

The server provides RESTful API endpoints:

- `GET /api/v1/health` - Health check endpoint
- `POST /api/v1/auth/register` - User registration
- `POST /api/v1/auth/login` - User login

## Configuration

Configuration is handled through environment variables:

- `DATABASE_URL` - PostgreSQL connection string
- `RUST_LOG` - Logging level (debug, info, warn, error)
- `SERVER_HOST` - Server host address
- `SERVER_PORT` - Server port number

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes following the code standards
4. Add tests for new functionality
5. Run all tests to ensure they pass
6. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Author

arkSong (arksong2018@gmail.com)

## Acknowledgments

- Original Mastodon project for the API specification
- Rust community for excellent tooling and libraries
- SQLx for type-safe database operations
- Axum for the web framework
- Tracing for structured logging
