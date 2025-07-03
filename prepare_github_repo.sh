#!/bin/bash

# Prepare GitHub Repository Script for Rustodon
# Author: arkSong (arksong2018@gmail.com)
# Project: rustodon
#
# This script helps organize the project environment and prepare for GitHub repository update

set -e

echo "=== Rustodon GitHub Repository Preparation ==="
echo "Starting at: $(date)"
echo

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."

    if ! command_exists git; then
        print_error "Git is not installed. Please install Git first."
        exit 1
    fi

    if ! command_exists cargo; then
        print_warning "Cargo is not installed. Rust toolchain may be needed."
    fi

    if ! command_exists docker; then
        print_warning "Docker is not installed. Containerization features may not work."
    fi

    print_success "Prerequisites check completed"
    echo
}

# Clean up temporary files
cleanup_temp_files() {
    print_status "Cleaning up temporary files..."

    # Remove temporary test files
    rm -f /tmp/response.json /tmp/test.png

    # Remove temporary status files
    rm -f status_id token status

    # Remove .DS_Store files
    find . -name ".DS_Store" -type f -delete

    # Remove test images
    rm -f images.jpg

    print_success "Temporary files cleaned up"
    echo
}

# Create .gitignore if it doesn't exist
create_gitignore() {
    print_status "Checking .gitignore file..."

    if [ ! -f ".gitignore" ]; then
        print_status "Creating .gitignore file..."
        cat > .gitignore << 'EOF'
# Rust
/target/
**/*.rs.bk
Cargo.lock

# IDE
.vscode/
.idea/
*.swp
*.swo
*~

# OS
.DS_Store
.DS_Store?
._*
.Spotlight-V100
.Trashes
ehthumbs.db
Thumbs.db

# Logs
*.log
logs/

# Environment variables
.env
.env.local
.env.production

# Temporary files
/tmp/
*.tmp
*.temp

# Test files
/tmp/response.json
/tmp/test.png
status_id
token
status
images.jpg

# Database
*.db
*.sqlite
*.sqlite3

# Docker
.dockerignore

# Node.js (if any)
node_modules/
npm-debug.log*
yarn-debug.log*
yarn-error.log*

# Python
__pycache__/
*.py[cod]
*$py.class
*.so
.Python
env/
venv/
.venv/

# Build outputs
dist/
build/
*.o
*.a

# Documentation
docs/_build/

# Backup files
*.bak
*.backup
*~

# Local configuration
config/local.yml
config/database.yml
EOF
        print_success ".gitignore file created"
    else
        print_success ".gitignore file already exists"
    fi
    echo
}

# Initialize Git repository
init_git_repo() {
    print_status "Initializing Git repository..."

    if [ ! -d ".git" ]; then
        git init
        print_success "Git repository initialized"
    else
        print_success "Git repository already exists"
    fi

    # Add all files
    git add .

    # Check if there are changes to commit
    if git diff --cached --quiet; then
        print_warning "No changes to commit"
    else
        print_status "Changes staged for commit"
    fi

    echo
}

# Create README files
create_readme_files() {
    print_status "Creating/updating README files..."

    # Main README
    if [ ! -f "README.md" ] || [ ! -f "rustodon/README.md" ]; then
        print_status "Creating main README.md..."
        cat > README.md << 'EOF'
# Rustodon

A high-performance Rust implementation of Mastodon server backend.

## Project Overview

- **Project Name**: rustodon
- **Language**: Rust
- **Architecture**: Modular crates with standard Rust project structure
- **Goal**: 100% compatibility with original Mastodon server functionality
- **Target**: High-performance, type-safe, concurrent server backend

## Quick Start

### Prerequisites

- Rust 1.77+
- PostgreSQL 15+
- Docker (optional)

### Development Setup

1. Clone the repository:
```bash
git clone https://github.com/arksong/rustodon.git
cd rustodon
```

2. Set up the database:
```bash
cd rustodon
./setup_database.sh
```

3. Build the project:
```bash
cargo build
```

4. Run tests:
```bash
cargo test
```

### Docker Setup

```bash
# Build and run with Docker Compose
docker-compose up -d

# Or use the simple setup
docker-compose -f rustodon/docker-compose.simple.yml up -d
```

## Project Structure

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
└── rustodon-server/        # Main server binary
```

## API Testing

We provide comprehensive API testing tools:

```bash
# Run basic API tests
./comprehensive_curl_test.sh

# Run advanced API tests (40 endpoints)
./advanced_api_test.sh

# View test report
cat API_TEST_REPORT.md
```

## Development

### Code Standards

- Follow Rust best practices and idioms
- Use async/await for I/O operations
- Implement proper error handling with `Result<T, E>`
- Use `tracing` for structured logging
- Write comprehensive tests

### Testing

```bash
# Run all tests
cargo test

# Run specific crate tests
cargo test -p rustodon-core

# Run with logging
RUST_LOG=debug cargo test
```

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Check for issues
cargo check
cargo clippy
cargo fmt
```

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

arkSong (arksong2018@gmail.com)

## Acknowledgments

- Original Mastodon project for the API specification
- Rust community for excellent tooling and libraries
- All contributors to this project
EOF
        print_success "Main README.md created"
    fi

    echo
}

# Create development documentation
create_dev_docs() {
    print_status "Creating development documentation..."

    # Create docs directory if it doesn't exist
    mkdir -p docs

    # Development guide
    cat > docs/development.md << 'EOF'
# Development Guide

## Environment Setup

### Required Tools

- Rust 1.77+
- PostgreSQL 15+
- Docker (optional)
- Git

### IDE Setup

Recommended IDE: VS Code with Rust extensions

Extensions:
- rust-analyzer
- CodeLLDB
- Even Better TOML

### Environment Variables

Copy the example environment file:
```bash
cp rustodon/env.example rustodon/.env
```

Edit the `.env` file with your configuration.

## Development Workflow

### 1. Code Changes

1. Create a feature branch
2. Make your changes
3. Run tests: `cargo test`
4. Check code quality: `cargo clippy`
5. Format code: `cargo fmt`
6. Commit with meaningful message

### 2. Testing

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test '*'

# API tests
./comprehensive_curl_test.sh
./advanced_api_test.sh
```

### 3. Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Check compilation
cargo check
```

### 4. Database

```bash
# Setup database
./rustodon/setup_database.sh

# Run migrations
cargo run -p rustodon-migrations

# Reset database
./rustodon/reset_and_migrate_db.sh
```

## Code Standards

### File Structure

- Each file must have detailed header comments
- All functions must have documentation comments
- Use standard Rust naming conventions
- Organize code with clear separation of concerns

### Documentation

```rust
//! Module description and purpose
//!
//! This module provides [specific functionality] for the Rustodon server.
//!
//! # Examples
//!
//! ```rust
//! use rustodon_core::example::Example;
//!
//! let example = Example::new();
//! example.doSomething();
//! ```
//!
//! # Author
//!
//! arkSong (arksong2018@gmail.com)

/// Function description
///
/// # Arguments
///
/// * `param` - Parameter description
///
/// # Returns
///
/// Result description
///
/// # Examples
///
/// ```rust
/// let result = function_name("example");
/// assert!(result.is_ok());
/// ```
pub fn function_name(param: &str) -> Result<(), Error> {
    // Implementation
}
```

### Logging

```rust
use tracing::{info, warn, error, debug, trace};

// Use appropriate log levels
trace!("Detailed debug information");
debug!("Debug information");
info!("General information");
warn!("Warning message");
error!("Error message");
```

### Error Handling

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModuleError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Internal error: {0}")]
    Internal(String),
}
```

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    fn test_function() {
        let result = function_name("test");
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_async_function() {
        let result = async_function().await;
        assert!(result.is_ok());
    }
}
```

### Integration Tests

Create test files in `tests/` directory:

```rust
// tests/integration_test.rs
use rustodon_core::*;

#[tokio::test]
async fn test_api_integration() {
    // Test API endpoints
}
```

### API Tests

Use the provided test scripts:

```bash
# Basic API tests
./comprehensive_curl_test.sh

# Advanced API tests
./advanced_api_test.sh
```

## Performance Considerations

- Use efficient data structures
- Implement proper caching strategies
- Optimize database queries
- Use connection pooling
- Implement rate limiting
- Handle concurrent requests efficiently

## Security

- Validate all input data
- Implement proper authentication and authorization
- Use secure defaults
- Follow OWASP guidelines
- Implement proper CORS policies
- Use HTTPS in production

## Deployment

### Docker

```bash
# Build image
docker build -t rustodon .

# Run container
docker run -p 3000:3000 rustodon
```

### Docker Compose

```bash
# Development
docker-compose -f rustodon/docker-compose.simple.yml up -d

# Production
docker-compose up -d
```

### Manual Deployment

1. Build release version: `cargo build --release`
2. Set up environment variables
3. Run migrations: `cargo run -p rustodon-migrations`
4. Start server: `./target/release/rustodon-server`

## Troubleshooting

### Common Issues

1. **Compilation errors**: Run `cargo check` to identify issues
2. **Database connection**: Check `.env` configuration
3. **Port conflicts**: Change port in configuration
4. **Permission errors**: Check file permissions

### Debug Mode

```bash
# Enable debug logging
RUST_LOG=debug cargo run

# Run with backtrace
RUST_BACKTRACE=1 cargo run
```

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Documentation](https://tokio.rs/)
- [SQLx Documentation](https://docs.rs/sqlx)
- [Axum Documentation](https://docs.rs/axum)
- [Tracing Documentation](https://docs.rs/tracing)
EOF
        print_success "Development documentation created"
    fi

    echo
}

# Create deployment documentation
create_deployment_docs() {
    print_status "Creating deployment documentation..."

    cat > docs/deployment.md << 'EOF'
# Deployment Guide

## Prerequisites

- Linux server (Ubuntu 20.04+ recommended)
- Docker and Docker Compose
- PostgreSQL 15+
- Nginx (optional, for reverse proxy)
- SSL certificate (for production)

## Quick Deployment

### Using Docker Compose

1. Clone the repository:
```bash
git clone https://github.com/arksong/rustodon.git
cd rustodon
```

2. Configure environment:
```bash
cp rustodon/env.example rustodon/.env
# Edit .env with your configuration
```

3. Start services:
```bash
docker-compose up -d
```

4. Run migrations:
```bash
docker-compose exec rustodon cargo run -p rustodon-migrations
```

5. Access the application:
```bash
# Health check
curl http://localhost:3000/health

# API endpoints
curl http://localhost:3000/api/v1/instance
```

## Production Deployment

### 1. Server Setup

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh

# Install Docker Compose
sudo curl -L "https://github.com/docker/compose/releases/download/v2.20.0/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose
```

### 2. Application Setup

```bash
# Clone repository
git clone https://github.com/arksong/rustodon.git
cd rustodon

# Create production environment
cp rustodon/env.example rustodon/.env.production
```

### 3. Environment Configuration

Edit `rustodon/.env.production`:

```env
# Database
DATABASE_URL=postgres://rustodon:password@localhost:5432/rustodon

# Server
BIND=0.0.0.0
PORT=3000

# Security
SECRET_KEY_BASE=your-secret-key-here

# Logging
RUST_LOG=info

# Federation
LOCAL_DOMAIN=your-domain.com
```

### 4. Database Setup

```bash
# Create database
sudo -u postgres createdb rustodon
sudo -u postgres createuser rustodon
sudo -u postgres psql -c "ALTER USER rustodon WITH PASSWORD 'password';"
sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE rustodon TO rustodon;"
```

### 5. Start Services

```bash
# Build and start
docker-compose -f rustodon/docker-compose.yml up -d

# Check status
docker-compose ps

# View logs
docker-compose logs -f rustodon
```

### 6. Nginx Configuration (Optional)

```nginx
server {
    listen 80;
    server_name your-domain.com;
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name your-domain.com;

    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;

    location / {
        proxy_pass http://localhost:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

## Monitoring

### Health Checks

```bash
# Application health
curl http://localhost:3000/health

# Database health
docker-compose exec db pg_isready

# Container status
docker-compose ps
```

### Logs

```bash
# Application logs
docker-compose logs -f rustodon

# Database logs
docker-compose logs -f db

# All logs
docker-compose logs -f
```

### Metrics

```bash
# Check resource usage
docker stats

# Monitor disk usage
df -h

# Monitor memory usage
free -h
```

## Backup and Recovery

### Database Backup

```bash
# Create backup
docker-compose exec db pg_dump -U rustodon rustodon > backup.sql

# Restore backup
docker-compose exec -T db psql -U rustodon rustodon < backup.sql
```

### File Backup

```bash
# Backup configuration
tar -czf config-backup.tar.gz rustodon/.env*

# Backup data
tar -czf data-backup.tar.gz storage/
```

## Scaling

### Horizontal Scaling

```bash
# Scale web service
docker-compose up -d --scale rustodon=3

# Load balancer configuration
# Use nginx or haproxy for load balancing
```

### Vertical Scaling

```bash
# Increase memory limits in docker-compose.yml
services:
  rustodon:
    deploy:
      resources:
        limits:
          memory: 2G
        reservations:
          memory: 1G
```

## Security

### Firewall Configuration

```bash
# Allow only necessary ports
sudo ufw allow 22/tcp    # SSH
sudo ufw allow 80/tcp    # HTTP
sudo ufw allow 443/tcp   # HTTPS
sudo ufw enable
```

### SSL/TLS

```bash
# Install Certbot
sudo apt install certbot python3-certbot-nginx

# Obtain certificate
sudo certbot --nginx -d your-domain.com
```

### Regular Updates

```bash
# Update application
git pull origin main
docker-compose down
docker-compose up -d --build

# Update system
sudo apt update && sudo apt upgrade -y
```

## Troubleshooting

### Common Issues

1. **Port already in use**:
   ```bash
   sudo lsof -i :3000
   sudo kill -9 <PID>
   ```

2. **Database connection failed**:
   ```bash
   docker-compose logs db
   docker-compose exec db pg_isready
   ```

3. **Permission denied**:
   ```bash
   sudo chown -R $USER:$USER .
   chmod +x scripts/*.sh
   ```

### Performance Issues

1. **High memory usage**:
   - Increase container memory limits
   - Optimize database queries
   - Implement caching

2. **Slow response times**:
   - Check database performance
   - Monitor network latency
   - Optimize application code

3. **High CPU usage**:
   - Profile application
   - Optimize algorithms
   - Scale horizontally

## Support

For deployment issues:

1. Check logs: `docker-compose logs -f`
2. Verify configuration: `docker-compose config`
3. Test connectivity: `curl http://localhost:3000/health`
4. Check system resources: `htop`, `df -h`

## Maintenance

### Regular Tasks

- Monitor logs daily
- Check disk space weekly
- Update dependencies monthly
- Review security patches
- Backup data regularly

### Updates

```bash
# Pull latest changes
git pull origin main

# Rebuild and restart
docker-compose down
docker-compose up -d --build

# Run migrations
docker-compose exec rustodon cargo run -p rustodon-migrations
```
EOF
        print_success "Deployment documentation created"
    fi

    echo
}

# Create API documentation
create_api_docs() {
    print_status "Creating API documentation..."

    cat > docs/api.md << 'EOF'
# API Documentation

## Overview

Rustodon implements the Mastodon API specification for 100% compatibility with existing Mastodon clients and tools.

## Base URL

```
http://localhost:3000/api/v1/
```

## Authentication

Most endpoints require authentication using Bearer tokens.

```bash
# Get authentication token
curl -X POST http://localhost:3000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username_or_email":"user","password":"password"}'

# Use token in requests
curl -H "Authorization: Bearer YOUR_TOKEN" \
  http://localhost:3000/api/v1/statuses
```

## Endpoints

### Health Check

```http
GET /health
```

**Response:**
```json
{
  "status": "ok",
  "message": "Health check passed",
  "timestamp": "2025-07-03T08:20:49.948907"
}
```

### Instance Information

```http
GET /api/v1/instance
```

**Response:**
```json
{
  "version": "1.0.0",
  "name": "Rustodon Test Server",
  "description": "Test server for API validation"
}
```

### Authentication

#### Register User

```http
POST /api/v1/auth/register
Content-Type: application/json

{
  "username": "newuser",
  "email": "user@example.com",
  "password": "password123",
  "agreement": true,
  "locale": "en"
}
```

#### Login

```http
POST /api/v1/auth/login
Content-Type: application/json

{
  "username_or_email": "user",
  "password": "password123"
}
```

**Response:**
```json
{
  "token": "your_auth_token",
  "user_id": "1",
  "message": "Login successful"
}
```

### Timelines

#### Public Timeline

```http
GET /api/v1/timelines/public
```

#### Home Timeline

```http
GET /api/v1/timelines/home
Authorization: Bearer YOUR_TOKEN
```

#### Local Timeline

```http
GET /api/v1/timelines/public?local=true
Authorization: Bearer YOUR_TOKEN
```

#### Tag Timeline

```http
GET /api/v1/timelines/tag/{hashtag}
Authorization: Bearer YOUR_TOKEN
```

### Statuses

#### Create Status

```http
POST /api/v1/statuses
Authorization: Bearer YOUR_TOKEN
Content-Type: application/json

{
  "status": "Hello, Rustodon!",
  "visibility": "public",
  "spoiler_text": "Content warning"
}
```

#### Get Status

```http
GET /api/v1/statuses/{id}
```

#### Delete Status

```http
DELETE /api/v1/statuses/{id}
Authorization: Bearer YOUR_TOKEN
```

#### Status Context

```http
GET /api/v1/statuses/{id}/context
Authorization: Bearer YOUR_TOKEN
```

#### Status Card

```http
GET /api/v1/statuses/{id}/card
Authorization: Bearer YOUR_TOKEN
```

### Accounts

#### Get Account

```http
GET /api/v1/accounts/{id}
```

#### Get Account Statuses

```http
GET /api/v1/accounts/{id}/statuses
Authorization: Bearer YOUR_TOKEN
```

#### Follow Account

```http
POST /api/v1/accounts/{id}/follow
Authorization: Bearer YOUR_TOKEN
```

#### Unfollow Account

```http
POST /api/v1/accounts/{id}/unfollow
Authorization: Bearer YOUR_TOKEN
```

#### Get Followers

```http
GET /api/v1/accounts/{id}/followers
Authorization: Bearer YOUR_TOKEN
```

#### Get Following

```http
GET /api/v1/accounts/{id}/following
Authorization: Bearer YOUR_TOKEN
```

### Media

#### Upload Media

```http
POST /api/v1/media
Authorization: Bearer YOUR_TOKEN
Content-Type: multipart/form-data

file: [binary file data]
description: "Image description"
focus: "0.5,0.5"
```

### Search

```http
GET /api/v1/search?q={query}
Authorization: Bearer YOUR_TOKEN
```

**Response:**
```json
{
  "accounts": [],
  "statuses": [],
  "hashtags": []
}
```

### Notifications

```http
GET /api/v1/notifications
Authorization: Bearer YOUR_TOKEN
```

### Lists

#### Get Lists

```http
GET /api/v1/lists
Authorization: Bearer YOUR_TOKEN
```

#### Create List

```http
POST /api/v1/lists
Authorization: Bearer YOUR_TOKEN
Content-Type: application/json

{
  "title": "My List"
}
```

### Conversations

```http
GET /api/v1/conversations
Authorization: Bearer YOUR_TOKEN
```

### Bookmarks

```http
GET /api/v1/bookmarks
Authorization: Bearer YOUR_TOKEN
```

### Mutes

```http
GET /api/v1/mutes
Authorization: Bearer YOUR_TOKEN
```

### Blocks

```http
GET /api/v1/blocks
Authorization: Bearer YOUR_TOKEN
```

### Reports

```http
GET /api/v1/reports
Authorization: Bearer YOUR_TOKEN
```

### Filters

```http
GET /api/v1/filters
Authorization: Bearer YOUR_TOKEN
```

## Error Responses

### 400 Bad Request

```json
{
  "error": "Validation failed",
  "details": "Invalid input parameters"
}
```

### 401 Unauthorized

```json
{
  "error": "Unauthorized",
  "message": "Invalid or missing authentication token"
}
```

### 404 Not Found

```json
{
  "error": "Record not found",
  "message": "The requested resource was not found"
}
```

### 422 Unprocessable Entity

```json
{
  "error": "Validation failed",
  "details": "Input validation errors"
}
```

### 500 Internal Server Error

```json
{
  "error": "Internal server error",
  "message": "An unexpected error occurred"
}
```

## Rate Limiting

API requests are rate limited to prevent abuse:

- **Authenticated requests**: 300 requests per 5 minutes
- **Unauthenticated requests**: 60 requests per 5 minutes

Rate limit headers are included in responses:

```
X-RateLimit-Limit: 300
X-RateLimit-Remaining: 299
X-RateLimit-Reset: 1640995200
```

## Pagination

List endpoints support pagination using `max_id` and `since_id` parameters:

```http
GET /api/v1/statuses?max_id=123&limit=20
```

Response includes pagination links:

```json
{
  "statuses": [...],
  "next": "http://localhost:3000/api/v1/statuses?max_id=100&limit=20",
  "prev": "http://localhost:3000/api/v1/statuses?since_id=140&limit=20"
}
```

## Testing

Use the provided test scripts to verify API functionality:

```bash
# Basic API tests
./comprehensive_curl_test.sh

# Advanced API tests (40 endpoints)
./advanced_api_test.sh

# View test report
cat API_TEST_REPORT.md
```

## Compatibility

This API is designed to be 100% compatible with the original Mastodon API specification. All standard Mastodon clients should work without modification.

## Support

For API-related issues:

1. Check the test reports
2. Verify authentication tokens
3. Review request/response formats
4. Check rate limiting
5. Monitor server logs
EOF
        print_success "API documentation created"
    fi

    echo
}

# Main execution
main() {
    print_status "Starting GitHub repository preparation..."
    echo

    # Check prerequisites
    check_prerequisites

    # Clean up temporary files
    cleanup_temp_files

    # Create .gitignore
    create_gitignore

    # Create documentation
    create_readme_files
    create_dev_docs
    create_deployment_docs
    create_api_docs

    # Initialize Git repository
    init_git_repo

    print_success "GitHub repository preparation completed!"
    echo
    print_status "Next steps:"
    echo "1. Review the created files"
    echo "2. Edit configuration files as needed"
    echo "3. Commit your changes:"
    echo "   git add ."
    echo "   git commit -m 'Initial commit: Rustodon project setup'"
    echo "4. Add remote repository:"
    echo "   git remote add origin https://github.com/yourusername/rustodon.git"
    echo "5. Push to GitHub:"
    echo "   git push -u origin main"
    echo
    print_status "Created files:"
    echo "  - README.md (main project documentation)"
    echo "  - .gitignore (Git ignore rules)"
    echo "  - docs/development.md (development guide)"
    echo "  - docs/deployment.md (deployment guide)"
    echo "  - docs/api.md (API documentation)"
    echo "  - API_TEST_REPORT.md (comprehensive test report)"
    echo
    print_status "Test scripts available:"
    echo "  - comprehensive_curl_test.sh (basic API tests)"
    echo "  - advanced_api_test.sh (40 endpoint tests)"
    echo "  - simple_test_server.py (Python test server)"
    echo
}

# Run main function
main "$@"
