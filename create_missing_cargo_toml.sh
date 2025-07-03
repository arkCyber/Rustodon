#!/bin/bash

# Create Missing Cargo.toml Script
# Author: arkSong (arksong2018@gmail.com)
# Project: rustodon
# Purpose: Create basic Cargo.toml files for crates that are missing them

set -e

echo "=== Creating Missing Cargo.toml Files ==="
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

# Function to create Cargo.toml for a crate
create_cargo_toml() {
    local crate_path="$1"
    local crate_name=$(basename "$crate_path")

    print_status "Creating Cargo.toml for $crate_name"

    cat > "$crate_path/Cargo.toml" << EOF
[package]
name = "$crate_name"
version = "0.1.0"
edition = "2021"
authors = ["arkSong <arksong2018@gmail.com>"]
description = "$crate_name module for Rustodon"
license = "MIT"
repository = "https://github.com/arkCyber/Rustodon"
keywords = ["mastodon", "activitypub", "social", "federation"]
categories = ["social-networking"]

[dependencies]
# Core dependencies
tokio = { version = "1.0", features = ["full"] }
tracing = "0.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
anyhow = "1.0"
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
futures = "0.3"
async-trait = "0.1"

# Internal dependencies
rustodon-core = { path = "../../core/rustodon-core" }
EOF

    print_success "Created Cargo.toml for $crate_name"
}

# List of crates missing Cargo.toml
missing_crates=(
    "crates/admin/rustodon-admin"
    "crates/api/rustodon-activitypub"
    "crates/api/rustodon-api"
    "crates/database/rustodon-db"
    "crates/features/rustodon-custom-emojis"
    "crates/features/rustodon-instances"
    "crates/features/rustodon-polls"
    "crates/features/rustodon-reports"
    "crates/features/rustodon-scheduled-statuses"
    "crates/features/rustodon-trends"
    "crates/utils/rustodon-config"
)

# Create Cargo.toml for each missing crate
for crate_path in "${missing_crates[@]}"; do
    if [ -d "$crate_path" ]; then
        create_cargo_toml "$crate_path"
    else
        print_warning "Crate directory not found: $crate_path"
    fi
done

print_success "All missing Cargo.toml files created!"
echo
print_status "Summary:"
echo "  - Created Cargo.toml for ${#missing_crates[@]} crates"
echo "  - All crates now have basic package configuration"
echo
print_status "Next steps:"
echo "1. Test build: cargo check"
echo "2. Review and customize Cargo.toml files as needed"
echo "3. Add specific dependencies for each crate"
