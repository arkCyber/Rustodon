#!/bin/bash

# Fix Cargo.toml Paths Script
# Author: arkSong (arksong2018@gmail.com)
# Project: rustodon
# Purpose: Fix rustodon-core path references in all Cargo.toml files

set -e

echo "=== Fixing Cargo.toml Paths ==="
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

# Function to fix paths in a Cargo.toml file
fix_cargo_paths() {
    local cargo_file="$1"
    local crate_dir=$(dirname "$cargo_file")

    print_status "Fixing paths in $(basename "$crate_dir")/Cargo.toml"

    # Calculate the correct path to rustodon-core
    local relative_path=""
    case "$crate_dir" in
        crates/core/*)
            relative_path="../rustodon-core"
            ;;
        crates/api/*|crates/auth/*|crates/database/*|crates/admin/*|crates/federation/*|crates/media/*|crates/utils/*|crates/cli/*|crates/server/*)
            relative_path="../../core/rustodon-core"
            ;;
        crates/features/*)
            relative_path="../../core/rustodon-core"
            ;;
        *)
            relative_path="../../core/rustodon-core"
            ;;
    esac

    # Replace the path in the file
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        sed -i '' "s|rustodon-core = { path = \".*rustodon-core\"}|rustodon-core = { path = \"$relative_path\"}|g" "$cargo_file"
    else
        # Linux
        sed -i "s|rustodon-core = { path = \".*rustodon-core\"}|rustodon-core = { path = \"$relative_path\"}|g" "$cargo_file"
    fi

    print_success "Fixed paths in $(basename "$crate_dir")/Cargo.toml"
}

# Find all Cargo.toml files that reference rustodon-core
cargo_files=$(find crates/ -name "Cargo.toml" -exec grep -l "rustodon-core.*path" {} \;)

# Fix paths in each file
for cargo_file in $cargo_files; do
    if [ -f "$cargo_file" ]; then
        fix_cargo_paths "$cargo_file"
    fi
done

print_success "All Cargo.toml paths fixed!"
echo
print_status "Summary:"
echo "  - Fixed rustodon-core paths in $(echo "$cargo_files" | wc -l | tr -d ' ') Cargo.toml files"
echo "  - All crates now reference the correct rustodon-core location"
echo
print_status "Next steps:"
echo "1. Test build: cargo check"
echo "2. Verify all dependencies resolve correctly"
