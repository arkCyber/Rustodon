#!/bin/bash

# Stable Dependencies Script for Rustodon
# This script implements stable, compatible dependency versions across all crates
# Author: arkSong (arksong2018@gmail.com)

set -e

echo "🔄 Implementing stable dependency versions..."

# Clean up first
echo "🧹 Cleaning up previous builds..."
cargo clean
find . -name "Cargo.lock" -delete 2>/dev/null || true

# Update workspace dependencies to stable versions
echo "📦 Updating workspace dependencies to stable versions..."

cat > Cargo.toml << 'EOF'
[workspace]
resolver = "2"
members = [
    "rustodon-core",
    "rustodon-db",
    "rustodon-api",
    "rustodon-auth",
    "rustodon-activitypub",
    "rustodon-workers",
    "rustodon-search",
    "rustodon-mailer",
    "rustodon-admin",
    "rustodon-config",
    "rustodon-logging",
    "rustodon-metrics",
    "rustodon-cache",
    "rustodon-queue",
    "rustodon-storage",
    "rustodon-notifications",
    "rustodon-media",
    "rustodon-federation",
    "rustodon-webhooks",
    "rustodon-scheduler",
    "rustodon-migrations",
    "rustodon-cli",
    "rustodon-server",
    "rustodon-streaming",
    "rustodon-oauth",
    "rustodon-polls",
    "rustodon-bookmarks",
    "rustodon-reports",
    "rustodon-follow-requests",
    "rustodon-announcements",
    "rustodon-custom-emojis",
    "rustodon-trends",
    "rustodon-tags",
    "rustodon-conversations",
    "rustodon-mentions",
    "rustodon-favourites",
    "rustodon-reblogs",
    "rustodon-follows",
    "rustodon-filters",
    "rustodon-lists",
    "rustodon-groups",
    "rustodon-statuses",
    "rustodon-accounts",
    "rustodon-instances",
    "rustodon-domains",
    "rustodon-sessions",
    "rustodon-applications",
    "rustodon-access-tokens",
    "rustodon-access-grants",
    "rustodon-devices",
    "rustodon-encrypted-messages",
    "rustodon-preview-cards",
    "rustodon-scheduled-statuses",
    "rustodon-status-pins",
    "rustodon-account-notes",
    "rustodon-account-warnings",
    "rustodon-account-moderation-notes",
    "rustodon-account-deletion-requests",
    "rustodon-account-aliases",
    "rustodon-account-conversations",
    "rustodon-account-suggestions",
    "rustodon-annual-reports",
    "rustodon-terms-of-service",
    "rustodon-user-settings",
    "rustodon-web",
    "rustodon-mutes",
    "rustodon-blocks",
    "rustodon-ip-blocks",
    "rustodon-email-domain-blocks",
    "rustodon-canonical-email-blocks",
    "rustodon-appeals",
    "rustodon-bulk-imports",
    "rustodon-software-updates",
    "rustodon-severed-relationships",
    "rustodon-tag-follows",
    "rustodon-follow-recommendation-suppressions",
    "rustodon-webauthn-credentials"
]

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["arkSong <arksong2018@gmail.com>"]
description = "A Rust implementation of Mastodon server backend, aiming for 100% compatibility with original Mastodon server functionality"
license = "AGPL-3.0"
repository = "https://github.com/arkCyber/Rustodon"
keywords = ["mastodon", "activitypub", "federation", "social-network", "rust"]
categories = ["web-programming", "api-bindings", "social-networking"]

[workspace.dependencies]
# Core async runtime
tokio = { version = "1.35", features = ["full"] }

# Web framework - using stable versions
axum = "0.7.4"
tower = "0.4.13"
tower-http = { version = "0.5.0", features = ["cors", "trace", "compression-full", "limit"] }

# Logging - simplified approach
tracing = { version = "0.1.40", features = ["std"] }
tracing-subscriber = { version = "0.3.18", features = ["env-filter", "fmt", "local-time"] }

# Serialization
serde = { version = "1.0.195", features = ["derive"] }
serde_json = "1.0.111"

# Database
sqlx = { version = "0.7.3", features = ["runtime-tokio-rustls", "postgres", "chrono", "uuid"] }

# Error handling
thiserror = "1.0.56"
anyhow = "1.0.79"

# Utilities
uuid = { version = "1.7.0", features = ["v4", "serde"] }
chrono = { version = "0.4.31", features = ["serde"] }
bcrypt = "0.15.0"
jsonwebtoken = "9.2.0"
rand = "0.8.5"
config = "0.14.0"
clap = { version = "4.4.11", features = ["derive"] }
futures = "0.3.29"
async-trait = "0.1.77"

# Caching and storage
redis = { version = "0.24.0", features = ["tokio-comp"] }

# HTTP client
reqwest = { version = "0.11.23", features = ["json", "stream"] }

# URL and MIME handling
url = "2.5.0"
mime = "0.3.17"
mime_guess = "2.0.4"

# File system
tempfile = "3.8.1"
walkdir = "2.4.0"
glob = "0.3.1"

# Text processing
regex = "1.10.2"

# Concurrency and data structures
lazy_static = "1.4.0"
dashmap = "5.5.3"
parking_lot = "0.12.1"
crossbeam-channel = "0.5.8"
rayon = "1.8.0"

# CLI and UI
indicatif = "0.17.7"
console = "0.15.8"
colored = "2.1.0"

# Logging (fallback)
env_logger = "0.10.2"
log = "0.4.20"

# System
backtrace = "0.3.69"
num_cpus = "1.16.0"
libc = "0.2.150"
EOF

echo "✅ Workspace dependencies updated to stable versions"

# Function to update crate dependencies
update_crate_deps() {
    local crate_dir="$1"
    local crate_name="$2"

    if [ -f "$crate_dir/Cargo.toml" ]; then
        echo "📦 Updating $crate_name dependencies..."

        # Create a backup
        cp "$crate_dir/Cargo.toml" "$crate_dir/Cargo.toml.backup"

        # Start with package section
        cat > "$crate_dir/Cargo.toml" << EOF
[package]
name = "$crate_name"
version.workspace = true
edition.workspace = true
authors.workspace = true
description.workspace = true
license.workspace = true
repository.workspace = true
keywords.workspace = true
categories.workspace = true

[dependencies]
# Core dependencies
tokio.workspace = true
tracing.workspace = true
serde.workspace = true
serde_json.workspace = true
thiserror.workspace = true
anyhow.workspace = true
uuid.workspace = true
chrono.workspace = true
futures.workspace = true
async-trait.workspace = true
EOF

        # Add specific dependencies based on crate type
        case "$crate_name" in
            *api*|*server*|*web*)
                cat >> "$crate_dir/Cargo.toml" << EOF
axum.workspace = true
tower.workspace = true
tower-http.workspace = true
EOF
                ;;
            *db*|*migrations*)
                cat >> "$crate_dir/Cargo.toml" << EOF
sqlx.workspace = true
EOF
                ;;
            *auth*|*oauth*)
                cat >> "$crate_dir/Cargo.toml" << EOF
bcrypt.workspace = true
jsonwebtoken.workspace = true
rand.workspace = true
EOF
                ;;
            *cache*|*queue*)
                cat >> "$crate_dir/Cargo.toml" << EOF
redis.workspace = true
dashmap.workspace = true
parking_lot.workspace = true
crossbeam-channel.workspace = true
EOF
                ;;
            *mailer*|*federation*)
                cat >> "$crate_dir/Cargo.toml" << EOF
reqwest.workspace = true
url.workspace = true
EOF
                ;;
            *cli*)
                cat >> "$crate_dir/Cargo.toml" << EOF
clap.workspace = true
indicatif.workspace = true
console.workspace = true
colored.workspace = true
EOF
                ;;
            *config*)
                cat >> "$crate_dir/Cargo.toml" << EOF
config.workspace = true
EOF
                ;;
            *logging*)
                cat >> "$crate_dir/Cargo.toml" << EOF
tracing-subscriber.workspace = true
env_logger.workspace = true
log.workspace = true
EOF
                ;;
            *media*|*storage*)
                cat >> "$crate_dir/Cargo.toml" << EOF
mime.workspace = true
mime_guess.workspace = true
tempfile.workspace = true
walkdir.workspace = true
glob.workspace = true
EOF
                ;;
            *search*)
                cat >> "$crate_dir/Cargo.toml" << EOF
regex.workspace = true
rayon.workspace = true
EOF
                ;;
            *workers*|*scheduler*)
                cat >> "$crate_dir/Cargo.toml" << EOF
crossbeam-channel.workspace = true
rayon.workspace = true
EOF
                ;;
        esac

        # Add internal dependencies (only if not rustodon-core itself)
        if [ "$crate_name" != "rustodon-core" ]; then
            cat >> "$crate_dir/Cargo.toml" << EOF

# Internal dependencies
rustodon-core = { path = "../rustodon-core" }
EOF
        fi

        # Add specific internal dependencies based on crate
        case "$crate_name" in
            rustodon-api)
                cat >> "$crate_dir/Cargo.toml" << EOF
rustodon-db = { path = "../rustodon-db" }
rustodon-auth = { path = "../rustodon-auth" }
EOF
                ;;
            rustodon-server)
                cat >> "$crate_dir/Cargo.toml" << EOF
rustodon-api = { path = "../rustodon-api" }
rustodon-db = { path = "../rustodon-db" }
rustodon-auth = { path = "../rustodon-auth" }
rustodon-workers = { path = "../rustodon-workers" }
rustodon-config = { path = "../rustodon-config" }
rustodon-logging = { path = "../rustodon-logging" }
EOF
                ;;
        esac

        echo "✅ Updated $crate_name"
    fi
}

# Update all crates
echo "🔄 Updating all crate dependencies..."

# Core crates first
update_crate_deps "rustodon-core" "rustodon-core"
update_crate_deps "rustodon-db" "rustodon-db"
update_crate_deps "rustodon-config" "rustodon-config"
update_crate_deps "rustodon-logging" "rustodon-logging"

# API and server crates
update_crate_deps "rustodon-api" "rustodon-api"
update_crate_deps "rustodon-server" "rustodon-server"
update_crate_deps "rustodon-auth" "rustodon-auth"
update_crate_deps "rustodon-oauth" "rustodon-oauth"

# Other crates
for dir in rustodon-*/; do
    if [ -d "$dir" ] && [ "$dir" != "rustodon-core/" ] && [ "$dir" != "rustodon-db/" ] && [ "$dir" != "rustodon-api/" ] && [ "$dir" != "rustodon-server/" ] && [ "$dir" != "rustodon-auth/" ] && [ "$dir" != "rustodon-oauth/" ] && [ "$dir" != "rustodon-config/" ] && [ "$dir" != "rustodon-logging/" ]; then
        crate_name=$(basename "$dir")
        update_crate_deps "$dir" "$crate_name"
    fi
done

echo "✅ All crate dependencies updated"

# Test the build
echo "🧪 Testing build with stable dependencies..."
cargo check --workspace

if [ $? -eq 0 ]; then
    echo "✅ Build successful with stable dependencies!"
    echo "🚀 You can now run: cargo run -p rustodon-server"
else
    echo "❌ Build failed. Checking for specific issues..."
    cargo check --workspace 2>&1 | head -20
fi

echo "📋 Summary:"
echo "- Updated to stable dependency versions"
echo "- Used workspace resolver = 2 for better compatibility"
echo "- Simplified tracing configuration"
echo "- Used conservative versions of axum, tower, and other web dependencies"
