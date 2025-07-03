# Rustodon Development Guide

This guide covers development practices, code standards, and contribution guidelines for Rustodon.

## Development Environment Setup

### Prerequisites

- **Rust**: 1.70 or higher
- **Git**: For version control
- **PostgreSQL**: 13+ (for full database integration)
- **Redis**: 6.2+ (for caching and job queues)
- **IDE**: VS Code with rust-analyzer extension recommended

### Initial Setup

```bash
# Clone the repository
git clone https://github.com/arkCyber/Rustodon.git
cd Rustodon

# Install Rust toolchain components
rustup component add rustfmt clippy

# Copy environment configuration
cp .env.example .env

# Build the project
cargo build
```

## Project Structure

Rustodon follows a modular workspace architecture:

```
rustodon/
├── Cargo.toml              # Workspace configuration
├── .env.example            # Environment template
├── test_api.sh            # API testing script
├── docs/                  # Documentation
├── rustodon-core/         # Core types and traits
├── rustodon-api/          # HTTP API implementation
├── rustodon-auth/         # Authentication system
├── rustodon-db/           # Database operations
├── rustodon-server/       # Main server binary
└── [other crates]/        # Feature-specific modules
```

### Crate Organization

Each crate follows this structure:

```
rustodon-{module}/
├── Cargo.toml             # Crate configuration
├── src/
│   ├── lib.rs            # Public API
│   ├── models.rs         # Data structures
│   ├── handlers.rs       # Business logic
│   ├── errors.rs         # Error types
│   └── tests.rs          # Unit tests
└── README.md             # Crate documentation
```

## Code Standards

### Rust Style Guide

Follow the official Rust style guide and these additional conventions:

```rust
//! Crate-level documentation
//!
//! Brief description of the crate's purpose and functionality.
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use tracing::{info, warn, error, debug, trace};
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

/// Result type alias for this module
pub type Result<T> = std::result::Result<T, ModuleError>;

/// Main struct with comprehensive documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleStruct {
    /// Unique identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Optional description
    pub description: Option<String>,
}

impl ExampleStruct {
    /// Creates a new instance
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier
    /// * `name` - Human-readable name
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustodon_module::ExampleStruct;
    ///
    /// let example = ExampleStruct::new("1", "Test");
    /// assert_eq!(example.id, "1");
    /// ```
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        let id = id.into();
        let name = name.into();
        
        trace!("Creating ExampleStruct with id: {}, name: {}", id, name);
        
        Self {
            id,
            name,
            description: None,
        }
    }

    /// Validates the struct data
    pub fn validate(&self) -> Result<()> {
        if self.id.is_empty() {
            return Err(ModuleError::Validation("ID cannot be empty".to_string()));
        }
        
        if self.name.is_empty() {
            return Err(ModuleError::Validation("Name cannot be empty".to_string()));
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_example_struct() {
        let example = ExampleStruct::new("1", "Test");
        assert_eq!(example.id, "1");
        assert_eq!(example.name, "Test");
        assert!(example.description.is_none());
    }

    #[test]
    fn test_validate_success() {
        let example = ExampleStruct::new("1", "Test");
        assert!(example.validate().is_ok());
    }

    #[test]
    fn test_validate_empty_id() {
        let example = ExampleStruct::new("", "Test");
        assert!(example.validate().is_err());
    }
}
```

### Naming Conventions

- **Crates**: `rustodon-{feature}` (kebab-case)
- **Modules**: `snake_case`
- **Structs/Enums**: `PascalCase`
- **Functions/Variables**: `snake_case`
- **Constants**: `SCREAMING_SNAKE_CASE`

### Error Handling

Always use `Result<T, E>` for fallible operations:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Authentication failed")]
    Unauthorized,
    #[error("Resource not found: {resource}")]
    NotFound { resource: String },
    #[error("Validation failed: {message}")]
    Validation { message: String },
}

pub type ApiResult<T> = Result<T, ApiError>;
```

### Logging

Use structured logging with the `tracing` crate:

```rust
use tracing::{info, warn, error, debug, trace, instrument};

#[instrument(skip(db))]
pub async fn create_user(db: &Database, username: &str) -> ApiResult<User> {
    info!("Creating user: {}", username);
    
    // Validate input
    if username.is_empty() {
        warn!("Attempted to create user with empty username");
        return Err(ApiError::Validation {
            message: "Username cannot be empty".to_string(),
        });
    }
    
    // Database operation
    match db.create_user(username).await {
        Ok(user) => {
            info!("Successfully created user: {} (id: {})", username, user.id);
            Ok(user)
        }
        Err(e) => {
            error!("Failed to create user {}: {}", username, e);
            Err(ApiError::Database(e))
        }
    }
}
```

## Testing Strategy

### Unit Tests

Write comprehensive unit tests for all functions:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_create_user_success() {
        let db = MockDatabase::new();
        let result = create_user(&db, "testuser").await;
        assert!(result.is_ok());
    }

    #[test]
    async fn test_create_user_empty_username() {
        let db = MockDatabase::new();
        let result = create_user(&db, "").await;
        assert!(matches!(result, Err(ApiError::Validation { .. })));
    }
}
```

### Integration Tests

Create integration tests in the `tests/` directory:

```rust
// tests/api_integration.rs
use rustodon_api::*;
use tokio::test;

#[test]
async fn test_user_registration_flow() {
    let app = create_test_app().await;
    
    // Test registration
    let response = app
        .post("/api/v1/auth/register")
        .json(&serde_json::json!({
            "username": "testuser",
            "email": "test@example.com",
            "password": "password123"
        }))
        .send()
        .await;
    
    assert_eq!(response.status(), 201);
    
    // Test login
    let response = app
        .post("/api/v1/auth/login")
        .json(&serde_json::json!({
            "username": "testuser",
            "password": "password123"
        }))
        .send()
        .await;
    
    assert_eq!(response.status(), 200);
}
```

### API Testing

Use the provided curl test script:

```bash
# Make executable
chmod +x test_api.sh

# Run tests
./test_api.sh
```

## Development Workflow

### 1. Feature Development

```bash
# Create feature branch
git checkout -b feature/new-feature

# Make changes
# ... code changes ...

# Run tests
cargo test

# Format code
cargo fmt

# Check linting
cargo clippy

# Commit changes
git add .
git commit -m "feat: add new feature"

# Push branch
git push origin feature/new-feature
```

### 2. Code Review Process

1. Create pull request
2. Ensure CI passes
3. Request review from maintainers
4. Address feedback
5. Merge after approval

### 3. Release Process

1. Update version numbers
2. Update CHANGELOG.md
3. Create release tag
4. Deploy to production

## Database Development

### Using SQLx

```rust
use sqlx::{PgPool, query_as};

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
}

pub async fn get_user_by_id(pool: &PgPool, id: i64) -> Result<Option<User>, sqlx::Error> {
    let user = query_as!(
        User,
        "SELECT id, username, email FROM users WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await?;
    
    Ok(user)
}
```

### Offline Mode

For development without database:

```bash
export SQLX_OFFLINE=true
cargo build
```

## Performance Considerations

### Async Best Practices

```rust
use tokio::time::{timeout, Duration};

// Use timeouts for external calls
let result = timeout(
    Duration::from_secs(30),
    external_api_call()
).await??;

// Use spawn for CPU-intensive tasks
let handle = tokio::task::spawn_blocking(|| {
    // CPU-intensive work
    compute_heavy_operation()
});
let result = handle.await?;
```

### Memory Management

- Use `Arc<T>` for shared immutable data
- Use `Mutex<T>` or `RwLock<T>` for shared mutable data
- Prefer streaming for large datasets
- Use connection pooling for database operations

## Debugging

### Logging Configuration

```bash
# Enable debug logging
export RUST_LOG=debug

# Enable trace logging for specific module
export RUST_LOG=rustodon_api=trace

# Pretty print logs
export PRETTY_LOGS=true
```

### Common Issues

1. **Compilation errors**: Set `SQLX_OFFLINE=true`
2. **Port conflicts**: Server automatically handles port 3000
3. **Database connection**: Check PostgreSQL is running
4. **Redis connection**: Ensure Redis is accessible

## Contributing Guidelines

### Before Contributing

1. Read this development guide
2. Check existing issues and discussions
3. Follow the code standards
4. Write comprehensive tests
5. Update documentation

### Pull Request Guidelines

1. **Title**: Use conventional commits format
2. **Description**: Explain what and why
3. **Tests**: Include relevant tests
4. **Documentation**: Update docs if needed
5. **Breaking Changes**: Clearly mark and explain

### Code Review Checklist

- [ ] Code follows style guidelines
- [ ] Tests are comprehensive
- [ ] Documentation is updated
- [ ] No breaking changes without justification
- [ ] Performance impact considered
- [ ] Security implications reviewed

## Resources

- **Rust Book**: https://doc.rust-lang.org/book/
- **Async Book**: https://rust-lang.github.io/async-book/
- **SQLx Documentation**: https://docs.rs/sqlx/
- **Axum Documentation**: https://docs.rs/axum/
- **Tracing Documentation**: https://docs.rs/tracing/

## Getting Help

- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: Questions and general discussion
- **Email**: arksong2018@gmail.com for direct contact
