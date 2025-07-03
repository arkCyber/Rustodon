#!/bin/bash

# System Optimization Script for Rustodon High-Performance Server
# Optimizes system parameters for 10k+ concurrent users
# Author: arkSong (arksong2018@gmail.com)

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔧 Rustodon System Optimization Script${NC}"
echo -e "${BLUE}=====================================${NC}"
echo ""

# Check if running as root (required for some optimizations)
if [[ $EUID -eq 0 ]]; then
    echo -e "${YELLOW}⚠️  Running as root - will apply system-wide optimizations${NC}"
else
    echo -e "${YELLOW}ℹ️  Running as user - some optimizations may be limited${NC}"
fi
echo ""

# Function to apply Linux optimizations
optimize_linux() {
    echo -e "${BLUE}🐧 Applying Linux optimizations...${NC}"

    # Increase file descriptor limits
    if [[ $EUID -eq 0 ]]; then
        echo "* soft nofile 65536" >> /etc/security/limits.conf
        echo "* hard nofile 65536" >> /etc/security/limits.conf
        echo "root soft nofile 65536" >> /etc/security/limits.conf
        echo "root hard nofile 65536" >> /etc/security/limits.conf
        echo -e "${GREEN}✅ Increased file descriptor limits${NC}"
    fi

    # Optimize TCP settings
    if [[ $EUID -eq 0 ]]; then
        # Increase TCP buffer sizes
        echo "net.core.rmem_max = 16777216" >> /etc/sysctl.conf
        echo "net.core.wmem_max = 16777216" >> /etc/sysctl.conf
        echo "net.ipv4.tcp_rmem = 4096 87380 16777216" >> /etc/sysctl.conf
        echo "net.ipv4.tcp_wmem = 4096 65536 16777216" >> /etc/sysctl.conf

        # Optimize TCP connection handling
        echo "net.ipv4.tcp_congestion_control = bbr" >> /etc/sysctl.conf
        echo "net.ipv4.tcp_slow_start_after_idle = 0" >> /etc/sysctl.conf
        echo "net.ipv4.tcp_tw_reuse = 1" >> /etc/sysctl.conf
        echo "net.ipv4.tcp_fin_timeout = 15" >> /etc/sysctl.conf

        # Increase connection limits
        echo "net.core.somaxconn = 65535" >> /etc/sysctl.conf
        echo "net.ipv4.tcp_max_syn_backlog = 65535" >> /etc/sysctl.conf

        # Apply changes
        sysctl -p
        echo -e "${GREEN}✅ Applied TCP optimizations${NC}"
    fi

    # Optimize PostgreSQL settings (if running as root)
    if [[ $EUID -eq 0 ]] && command -v psql &> /dev/null; then
        echo -e "${YELLOW}🗄️  Optimizing PostgreSQL for high concurrency...${NC}"

        # Backup current config
        cp /etc/postgresql/*/main/postgresql.conf /etc/postgresql/*/main/postgresql.conf.backup.$(date +%Y%m%d_%H%M%S)

        # Update PostgreSQL configuration
        cat >> /etc/postgresql/*/main/postgresql.conf << EOF

# Rustodon High-Performance Optimizations
max_connections = 200
shared_buffers = 256MB
effective_cache_size = 1GB
maintenance_work_mem = 64MB
checkpoint_completion_target = 0.9
wal_buffers = 16MB
default_statistics_target = 100
random_page_cost = 1.1
effective_io_concurrency = 200
work_mem = 4MB
min_wal_size = 1GB
max_wal_size = 4GB
max_worker_processes = 8
max_parallel_workers_per_gather = 4
max_parallel_workers = 8
max_parallel_maintenance_workers = 4
EOF

        # Restart PostgreSQL
        systemctl restart postgresql
        echo -e "${GREEN}✅ PostgreSQL optimized for high concurrency${NC}"
    fi
}

# Function to apply macOS optimizations
optimize_macos() {
    echo -e "${BLUE}🍎 Applying macOS optimizations...${NC}"

    # Increase file descriptor limits
    echo "kern.maxfiles=65536" | sudo tee -a /etc/sysctl.conf
    echo "kern.maxfilesperproc=65536" | sudo tee -a /etc/sysctl.conf
    sudo sysctl -w kern.maxfiles=65536
    sudo sysctl -w kern.maxfilesperproc=65536
    echo -e "${GREEN}✅ Increased file descriptor limits${NC}"

    # Optimize TCP settings
    sudo sysctl -w net.inet.tcp.sendspace=65536
    sudo sysctl -w net.inet.tcp.recvspace=65536
    sudo sysctl -w net.inet.tcp.sendbuf_max=2097152
    sudo sysctl -w net.inet.tcp.recvbuf_max=2097152
    echo -e "${GREEN}✅ Applied TCP optimizations${NC}"

    # Optimize PostgreSQL settings (if using Homebrew)
    if command -v brew &> /dev/null && brew list | grep -q postgresql; then
        echo -e "${YELLOW}🗄️  Optimizing PostgreSQL for high concurrency...${NC}"

        PG_CONFIG_DIR=$(brew --prefix postgresql)/var/postgresql@14
        if [ -d "$PG_CONFIG_DIR" ]; then
            # Backup current config
            cp "$PG_CONFIG_DIR/postgresql.conf" "$PG_CONFIG_DIR/postgresql.conf.backup.$(date +%Y%m%d_%H%M%S)"

            # Update PostgreSQL configuration
            cat >> "$PG_CONFIG_DIR/postgresql.conf" << EOF

# Rustodon High-Performance Optimizations
max_connections = 200
shared_buffers = 256MB
effective_cache_size = 1GB
maintenance_work_mem = 64MB
checkpoint_completion_target = 0.9
wal_buffers = 16MB
default_statistics_target = 100
random_page_cost = 1.1
effective_io_concurrency = 200
work_mem = 4MB
min_wal_size = 1GB
max_wal_size = 4GB
max_worker_processes = 8
max_parallel_workers_per_gather = 4
max_parallel_workers = 8
max_parallel_maintenance_workers = 4
EOF

            # Restart PostgreSQL
            brew services restart postgresql
            echo -e "${GREEN}✅ PostgreSQL optimized for high concurrency${NC}"
        fi
    fi
}

# Function to create optimized Rustodon configuration
create_optimized_config() {
    echo -e "${BLUE}⚙️  Creating optimized Rustodon configuration...${NC}"

    # Create environment file with optimized settings
    cat > .env.optimized << EOF
# Rustodon High-Performance Configuration
# Optimized for 10k+ concurrent users

# Database Configuration
DATABASE_URL=postgres://rustodon:rustodon@localhost:5432/rustodon
DATABASE_MAX_CONNECTIONS=200
DATABASE_MIN_CONNECTIONS=50
DATABASE_CONNECT_TIMEOUT=5
DATABASE_STATEMENT_TIMEOUT=15

# Server Configuration
RUST_LOG=info
RUST_BACKTRACE=0

# Performance Settings
TOKIO_WORKER_THREADS=16
HTTP_MAX_CONNECTIONS=20000
HTTP_REQUEST_TIMEOUT=15
HTTP_KEEP_ALIVE_TIMEOUT=120
HTTP_MAX_BODY_SIZE=52428800

# Memory Settings
MALLOC_ARENA_MAX=2
RUST_MIN_STACK=2097152

# Network Settings
TCP_NODELAY=1
TCP_KEEPALIVE=1
EOF

    echo -e "${GREEN}✅ Created optimized configuration: .env.optimized${NC}"
}

# Function to create systemd service (Linux)
create_systemd_service() {
    if [[ "$OSTYPE" == "linux-gnu"* ]] && [[ $EUID -eq 0 ]]; then
        echo -e "${BLUE}🔧 Creating systemd service for Rustodon...${NC}"

        cat > /etc/systemd/system/rustodon.service << EOF
[Unit]
Description=Rustodon High-Performance Server
After=network.target postgresql.service
Wants=postgresql.service

[Service]
Type=simple
User=rustodon
Group=rustodon
WorkingDirectory=/opt/rustodon
Environment=RUST_LOG=info
Environment=RUST_BACKTRACE=0
Environment=MALLOC_ARENA_MAX=2
ExecStart=/opt/rustodon/target/release/rustodon-server
Restart=always
RestartSec=10
LimitNOFILE=65536
LimitNPROC=65536

# Performance optimizations
Nice=-10
IOSchedulingClass=1
IOSchedulingPriority=4

[Install]
WantedBy=multi-user.target
EOF

        systemctl daemon-reload
        systemctl enable rustodon.service
        echo -e "${GREEN}✅ Created systemd service: rustodon.service${NC}"
    fi
}

# Function to create launchd service (macOS)
create_launchd_service() {
    if [[ "$OSTYPE" == "darwin"* ]]; then
        echo -e "${BLUE}🔧 Creating launchd service for Rustodon...${NC}"

        cat > ~/Library/LaunchAgents/com.rustodon.server.plist << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.rustodon.server</string>
    <key>ProgramArguments</key>
    <array>
        <string>/usr/local/bin/rustodon-server</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>/var/log/rustodon.log</string>
    <key>StandardErrorPath</key>
    <string>/var/log/rustodon.error.log</string>
    <key>EnvironmentVariables</key>
    <dict>
        <key>RUST_LOG</key>
        <string>info</string>
        <key>RUST_BACKTRACE</key>
        <string>0</string>
        <key>MALLOC_ARENA_MAX</key>
        <string>2</string>
    </dict>
</dict>
</plist>
EOF

        launchctl load ~/Library/LaunchAgents/com.rustodon.server.plist
        echo -e "${GREEN}✅ Created launchd service: com.rustodon.server${NC}"
    fi
}

# Function to optimize Rust compilation
optimize_rust_compilation() {
    echo -e "${BLUE}🦀 Optimizing Rust compilation...${NC}"

    # Create .cargo/config.toml with optimizations
    mkdir -p .cargo
    cat > .cargo/config.toml << EOF
[build]
rustflags = ["-C", "target-cpu=native", "-C", "target-feature=+crt-static"]

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true

[profile.dev]
opt-level = 1
debug = true
EOF

    echo -e "${GREEN}✅ Created optimized Rust compilation config${NC}"
}

# Main optimization function
main() {
    echo -e "${BLUE}🚀 Starting system optimization for Rustodon...${NC}"
    echo ""

    # Detect operating system and apply appropriate optimizations
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        optimize_linux
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        optimize_macos
    else
        echo -e "${RED}❌ Unsupported operating system: $OSTYPE${NC}"
        exit 1
    fi

    # Create optimized configuration
    create_optimized_config

    # Create service files
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        create_systemd_service
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        create_launchd_service
    fi

    # Optimize Rust compilation
    optimize_rust_compilation

    echo ""
    echo -e "${GREEN}🎉 System optimization completed!${NC}"
    echo ""
    echo -e "${BLUE}📋 Next steps:${NC}"
    echo -e "1. Source the optimized environment: ${GREEN}source .env.optimized${NC}"
    echo -e "2. Build with optimizations: ${GREEN}cargo build --release${NC}"
    echo -e "3. Start the server: ${GREEN}cargo run -p rustodon-server --release${NC}"
    echo -e "4. Run performance tests: ${GREEN}./performance_test.sh${NC}"
    echo ""
    echo -e "${YELLOW}⚠️  Note: Some optimizations require a system restart to take full effect${NC}"
}

# Run main function
main "$@"
