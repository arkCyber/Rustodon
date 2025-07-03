#!/bin/bash

# System Optimization Script for Rustodon
echo "🔧 Rustodon System Optimization Script"
echo "====================================="

# Create optimized environment file
cat > .env.optimized << EOF
# Rustodon High-Performance Configuration
DATABASE_URL=postgres://rustodon:rustodon@localhost:5432/rustodon
DATABASE_MAX_CONNECTIONS=200
DATABASE_MIN_CONNECTIONS=50
DATABASE_CONNECT_TIMEOUT=5
DATABASE_STATEMENT_TIMEOUT=15
RUST_LOG=info
RUST_BACKTRACE=0
TOKIO_WORKER_THREADS=16
HTTP_MAX_CONNECTIONS=20000
HTTP_REQUEST_TIMEOUT=15
HTTP_KEEP_ALIVE_TIMEOUT=120
HTTP_MAX_BODY_SIZE=52428800
MALLOC_ARENA_MAX=2
RUST_MIN_STACK=2097152
TCP_NODELAY=1
TCP_KEEPALIVE=1
EOF

echo "✅ Created optimized configuration: .env.optimized"

# Create optimized Rust compilation config
mkdir -p .cargo
cat > .cargo/config.toml << EOF
[build]
rustflags = ["-C", "target-cpu=native"]

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

echo "✅ Created optimized Rust compilation config"

echo ""
echo "🎉 System optimization completed!"
echo ""
echo "📋 Next steps:"
echo "1. Source the optimized environment: source .env.optimized"
echo "2. Build with optimizations: cargo build --release"
echo "3. Start the server: cargo run -p rustodon-server --release"
echo "4. Run performance tests: ./performance_test.sh"

