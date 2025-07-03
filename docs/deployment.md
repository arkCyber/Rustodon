# Rustodon Deployment Guide

This guide covers various deployment options for Rustodon.

## Prerequisites

- **Rust**: 1.70 or higher
- **PostgreSQL**: 13 or higher
- **Redis**: 6.2 or higher
- **Git**: For cloning the repository

## Local Development Deployment

### 1. Clone and Setup

```bash
# Clone the repository
git clone https://github.com/arkCyber/Rustodon.git
cd Rustodon

# Copy environment configuration
cp .env.example .env
```

### 2. Configure Environment

Edit the `.env` file with your settings:

```bash
# Essential settings for local development
DATABASE_URL=postgresql://rustodon:password@localhost/rustodon
REDIS_URL=redis://localhost:6379
RUSTODON_HOST=0.0.0.0
RUSTODON_PORT=3000
SQLX_OFFLINE=true
RUST_LOG=info
```

### 3. Install Dependencies

#### PostgreSQL
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install postgresql postgresql-contrib

# macOS (with Homebrew)
brew install postgresql
brew services start postgresql

# Create database and user
sudo -u postgres psql
CREATE USER rustodon WITH PASSWORD 'password';
CREATE DATABASE rustodon OWNER rustodon;
GRANT ALL PRIVILEGES ON DATABASE rustodon TO rustodon;
\q
```

#### Redis
```bash
# Ubuntu/Debian
sudo apt install redis-server

# macOS (with Homebrew)
brew install redis
brew services start redis
```

### 4. Build and Run

```bash
# Build the project
cargo build --release

# Run the server
SQLX_OFFLINE=true cargo run -p rustodon-server
```

### 5. Test the Deployment

```bash
# Make test script executable
chmod +x test_api.sh

# Run API tests
./test_api.sh
```

## Docker Deployment

### 1. Using Docker Compose (Recommended)

Create a `docker-compose.yml` file:

```yaml
version: '3.8'

services:
  rustodon:
    build: .
    ports:
      - "3000:3000"
    environment:
      - DATABASE_URL=postgresql://rustodon:password@db:5432/rustodon
      - REDIS_URL=redis://redis:6379
      - SQLX_OFFLINE=true
    depends_on:
      - db
      - redis
    volumes:
      - ./storage:/app/storage

  db:
    image: postgres:15
    environment:
      - POSTGRES_USER=rustodon
      - POSTGRES_PASSWORD=password
      - POSTGRES_DB=rustodon
    volumes:
      - postgres_data:/var/lib/postgresql/data
    ports:
      - "5432:5432"

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data

volumes:
  postgres_data:
  redis_data:
```

### 2. Build and Deploy

```bash
# Build and start services
docker-compose up -d

# View logs
docker-compose logs -f rustodon

# Stop services
docker-compose down
```

## Production Deployment

### 1. Server Requirements

**Minimum Requirements:**
- CPU: 2 cores
- RAM: 4GB
- Storage: 50GB SSD
- Network: 100 Mbps

**Recommended:**
- CPU: 4+ cores
- RAM: 8GB+
- Storage: 100GB+ SSD
- Network: 1 Gbps

### 2. Security Configuration

```bash
# Generate a secure secret key
openssl rand -hex 64

# Update .env with production settings
RUSTODON_DOMAIN=yourdomain.com
RUSTODON_SECRET_KEY_BASE=your-generated-secret-key
DEBUG_MODE=false
RUST_LOG=warn
```

### 3. Database Setup

```bash
# Create production database
sudo -u postgres createuser --createdb rustodon
sudo -u postgres createdb rustodon_production --owner=rustodon

# Set strong password
sudo -u postgres psql
ALTER USER rustodon PASSWORD 'strong-random-password';
\q
```

### 4. Reverse Proxy (Nginx)

Create `/etc/nginx/sites-available/rustodon`:

```nginx
server {
    listen 80;
    server_name yourdomain.com;
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name yourdomain.com;

    ssl_certificate /path/to/your/certificate.crt;
    ssl_certificate_key /path/to/your/private.key;

    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    location /api/v1/streaming {
        proxy_pass http://127.0.0.1:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }
}
```

### 5. Systemd Service

Create `/etc/systemd/system/rustodon.service`:

```ini
[Unit]
Description=Rustodon Server
After=network.target postgresql.service redis.service

[Service]
Type=simple
User=rustodon
Group=rustodon
WorkingDirectory=/opt/rustodon
Environment=SQLX_OFFLINE=true
ExecStart=/opt/rustodon/target/release/rustodon-server
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

Enable and start the service:

```bash
sudo systemctl daemon-reload
sudo systemctl enable rustodon
sudo systemctl start rustodon
sudo systemctl status rustodon
```

## Monitoring and Maintenance

### 1. Log Management

```bash
# View logs
sudo journalctl -u rustodon -f

# Log rotation
sudo logrotate /etc/logrotate.d/rustodon
```

### 2. Database Maintenance

```bash
# Backup database
pg_dump rustodon_production > backup_$(date +%Y%m%d).sql

# Restore database
psql rustodon_production < backup_20231201.sql
```

### 3. Updates

```bash
# Pull latest changes
git pull origin main

# Rebuild
cargo build --release

# Restart service
sudo systemctl restart rustodon
```

## Troubleshooting

### Common Issues

1. **Port already in use**: The server automatically kills processes on port 3000
2. **Database connection failed**: Check PostgreSQL is running and credentials are correct
3. **Redis connection failed**: Ensure Redis is running and accessible
4. **Compilation errors**: Set `SQLX_OFFLINE=true` to bypass database checks during build

### Performance Tuning

1. **Database**: Tune PostgreSQL settings for your workload
2. **Redis**: Configure appropriate memory limits
3. **Rust**: Use `--release` builds for production
4. **Nginx**: Enable gzip compression and caching

### Getting Help

- **Issues**: [GitHub Issues](https://github.com/arkCyber/Rustodon/issues)
- **Discussions**: [GitHub Discussions](https://github.com/arkCyber/Rustodon/discussions)
- **Email**: arksong2018@gmail.com
