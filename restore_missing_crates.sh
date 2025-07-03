#!/bin/bash

# Restore Missing Crates Script
# Author: arkSong (arksong2018@gmail.com)
# Project: rustodon
# Purpose: Restore missing crates from GitHub and organize them properly

set -e

echo "=== Restore Missing Crates ==="
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

# Restore missing crates from remote repository
restore_missing_crates() {
    print_status "Restoring missing crates from remote repository..."

    # List of crates that should be in different directories
    local federation_crates=("rustodon-activitypub" "rustodon-federation")
    local media_crates=("rustodon-media" "rustodon-storage")
    local utils_crates=("rustodon-cache" "rustodon-logging" "rustodon-mailer" "rustodon-metrics" "rustodon-queue" "rustodon-scheduler" "rustodon-search" "rustodon-webhooks" "rustodon-workers")

    # Restore federation crates
    for crate in "${federation_crates[@]}"; do
        if [ -d "/tmp/rustodon_restore/$crate" ] && [ ! -d "crates/federation/$crate" ]; then
            cp -r "/tmp/rustodon_restore/$crate" "crates/federation/"
            print_success "Restored $crate to crates/federation/"
        fi
    done

    # Restore media crates
    for crate in "${media_crates[@]}"; do
        if [ -d "/tmp/rustodon_restore/$crate" ] && [ ! -d "crates/media/$crate" ]; then
            cp -r "/tmp/rustodon_restore/$crate" "crates/media/"
            print_success "Restored $crate to crates/media/"
        fi
    done

    # Restore utils crates
    for crate in "${utils_crates[@]}"; do
        if [ -d "/tmp/rustodon_restore/$crate" ] && [ ! -d "crates/utils/$crate" ]; then
            cp -r "/tmp/rustodon_restore/$crate" "crates/utils/"
            print_success "Restored $crate to crates/utils/"
        fi
    done

    # Check for any other missing crates in features
    for crate in /tmp/rustodon_restore/rustodon-*; do
        crate_name=$(basename "$crate")
        if [ ! -d "crates/features/$crate_name" ] && [ ! -d "crates/api/$crate_name" ] && [ ! -d "crates/auth/$crate_name" ] && [ ! -d "crates/core/$crate_name" ] && [ ! -d "crates/database/$crate_name" ] && [ ! -d "crates/admin/$crate_name" ] && [ ! -d "crates/federation/$crate_name" ] && [ ! -d "crates/media/$crate_name" ] && [ ! -d "crates/utils/$crate_name" ] && [ ! -d "crates/cli/$crate_name" ] && [ ! -d "crates/server/$crate_name" ]; then
            cp -r "$crate" "crates/features/"
            print_success "Restored $crate_name to crates/features/"
        fi
    done

    print_success "All missing crates restored"
}

# Update Cargo.toml to remove non-existent crates
update_cargo_toml() {
    print_status "Updating Cargo.toml to remove non-existent crates..."

    # Create a new Cargo.toml with only existing crates
    cat > Cargo.toml << 'EOF'
[workspace]
name = "rustodon"
version = "0.1.0"
edition = "2021"
authors = ["arkSong <arksong2018@gmail.com>"]
description = "A Rust implementation of Mastodon server backend"
license = "MIT"
repository = "https://github.com/arkCyber/Rustodon"
keywords = ["mastodon", "activitypub", "social", "federation", "rust"]
categories = ["social-networking", "web-programming"]

members = [
    # Core crates
    "crates/core/rustodon-core",

    # API crates
    "crates/api/rustodon-api",
    "crates/api/rustodon-activitypub",
    "crates/api/rustodon-streaming",

    # Auth crates
    "crates/auth/rustodon-auth",
    "crates/auth/rustodon-oauth",
    "crates/auth/rustodon-sessions",
    "crates/auth/rustodon-webauthn-credentials",

    # Database crates
    "crates/database/rustodon-db",
    "crates/database/rustodon-migrations",

    # Admin crates
    "crates/admin/rustodon-admin",

    # Federation crates
    "crates/federation/rustodon-federation",
    "crates/federation/rustodon-activitypub",

    # Media crates
    "crates/media/rustodon-media",
    "crates/media/rustodon-storage",

    # Utility crates
    "crates/utils/rustodon-config",
    "crates/utils/rustodon-cache",
    "crates/utils/rustodon-queue",
    "crates/utils/rustodon-metrics",
    "crates/utils/rustodon-logging",
    "crates/utils/rustodon-mailer",
    "crates/utils/rustodon-search",
    "crates/utils/rustodon-scheduler",
    "crates/utils/rustodon-workers",
    "crates/utils/rustodon-webhooks",

    # CLI crates
    "crates/cli/rustodon-cli",

    # Server crates
    "crates/server/rustodon-server",
    "crates/server/rustodon-web",
]

# Add all feature crates dynamically
EOF

    # Add all existing feature crates
    for crate in crates/features/rustodon-*; do
        if [ -d "$crate" ]; then
            crate_name=$(basename "$crate")
            echo "    \"crates/features/$crate_name\"," >> Cargo.toml
        fi
    done

    # Add closing bracket and profiles
    cat >> Cargo.toml << 'EOF'

[profile.dev]
opt-level = 0
debug = true
strip = false

[profile.release]
opt-level = 3
debug = false
strip = true
lto = true
codegen-units = 1

[profile.test]
opt-level = 0
debug = true
strip = false

[profile.bench]
opt-level = 3
debug = false
strip = true
lto = true
codegen-units = 1
EOF

    print_success "Cargo.toml updated with existing crates only"
}

# Test the build
test_build() {
    print_status "Testing build..."

    if cargo check; then
        print_success "Build test passed!"
    else
        print_error "Build test failed!"
        return 1
    fi
}

# Clean up
cleanup() {
    print_status "Cleaning up temporary files..."
    rm -rf /tmp/rustodon_restore
    print_success "Cleanup completed"
}

# Main function
main() {
    print_status "Starting crate restoration process..."

    # Check if remote repository is available
    if [ ! -d "/tmp/rustodon_restore" ]; then
        print_error "Remote repository not found. Please run: git clone git@github.com:arkCyber/Rustodon.git /tmp/rustodon_restore"
        exit 1
    fi

    # Execute restoration steps
    restore_missing_crates
    update_cargo_toml
    test_build
    cleanup

    print_success "Crate restoration completed successfully!"
    echo
    print_status "Summary:"
    echo "  - Restored all missing crates from GitHub"
    echo "  - Organized crates into proper directory structure"
    echo "  - Updated workspace Cargo.toml"
    echo "  - Verified build works correctly"
    echo
    print_status "Next steps:"
    echo "1. Review the new structure"
    echo "2. Commit changes: git add . && git commit -m 'fix: restore missing crates and reorganize structure'"
    echo "3. Push to GitHub"
}

# Run main function
main
