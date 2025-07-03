#!/bin/bash

# Rustodon Project Organization Script
# Author: arkSong (arksong2018@gmail.com)
# Project: rustodon
# Purpose: Organize all rustodon crates into a structured crates/ directory

set -e

echo "=== Rustodon Project Organization ==="
echo "Starting at: $(date)"
echo "Purpose: Organize all rustodon crates into crates/ directory"
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

# Create crates directory structure
create_crates_structure() {
    print_status "Creating crates directory structure..."

    # Create main crates directory
    mkdir -p crates

    # Create subdirectories for better organization
    mkdir -p crates/core
    mkdir -p crates/api
    mkdir -p crates/auth
    mkdir -p crates/database
    mkdir -p crates/features
    mkdir -p crates/admin
    mkdir -p crates/media
    mkdir -p crates/federation
    mkdir -p crates/utils
    mkdir -p crates/cli
    mkdir -p crates/server

    print_success "Crates directory structure created"
}

# Move crates to appropriate subdirectories
move_crates() {
    print_status "Moving crates to organized structure..."

    # Core crates
    print_status "Moving core crates..."
    if [ -d "rustodon-core" ]; then
        mv rustodon-core crates/core/
        print_success "Moved rustodon-core to crates/core/"
    fi

    # API crates
    print_status "Moving API crates..."
    for crate in rustodon-api rustodon-activitypub rustodon-streaming; do
        if [ -d "$crate" ]; then
            mv "$crate" crates/api/
            print_success "Moved $crate to crates/api/"
        fi
    done

    # Auth crates
    print_status "Moving auth crates..."
    for crate in rustodon-auth rustodon-oauth rustodon-sessions rustodon-webauthn-credentials; do
        if [ -d "$crate" ]; then
            mv "$crate" crates/auth/
            print_success "Moved $crate to crates/auth/"
        fi
    done

    # Database crates
    print_status "Moving database crates..."
    for crate in rustodon-db rustodon-migrations; do
        if [ -d "$crate" ]; then
            mv "$crate" crates/database/
            print_success "Moved $crate to crates/database/"
        fi
    done

    # Feature crates
    print_status "Moving feature crates..."
    for crate in rustodon-accounts rustodon-statuses rustodon-notifications rustodon-follows rustodon-follow-requests rustodon-favourites rustodon-reblogs rustodon-bookmarks rustodon-lists rustodon-conversations rustodon-polls rustodon-media rustodon-tags rustodon-mentions rustodon-filters rustodon-reports rustodon-blocks rustodon-mutes rustodon-ip-blocks rustodon-email-domain-blocks rustodon-canonical-email-blocks rustodon-domain-blocks rustodon-custom-emojis rustodon-trends rustodon-announcements rustodon-featured-tags rustodon-endorsements rustodon-suggestions rustodon-directory rustodon-instances rustodon-peers rustodon-activity rustodon-annual-reports rustodon-appeals rustodon-bulk-imports rustodon-encrypted-messages rustodon-groups rustodon-severed-relationships rustodon-software-updates rustodon-status-pins rustodon-tag-follows rustodon-terms-of-service rustodon-user-settings rustodon-webhooks rustodon-workers rustodon-scheduler rustodon-queue rustodon-cache rustodon-storage rustodon-search rustodon-metrics rustodon-logging rustodon-mailer rustodon-federation rustodon-preview-cards rustodon-scheduled-statuses rustodon-access-grants rustodon-access-tokens rustodon-account-aliases rustodon-account-conversations rustodon-account-deletion-requests rustodon-account-moderation-notes rustodon-account-notes rustodon-account-suggestions rustodon-account-warnings rustodon-analytics rustodon-applications rustodon-devices rustodon-domains rustodon-follow-recommendation-suppressions; do
        if [ -d "$crate" ]; then
            mv "$crate" crates/features/
            print_success "Moved $crate to crates/features/"
        fi
    done

    # Admin crates
    print_status "Moving admin crates..."
    for crate in rustodon-admin; do
        if [ -d "$crate" ]; then
            mv "$crate" crates/admin/
            print_success "Moved $crate to crates/admin/"
        fi
    done

    # Media crates
    print_status "Moving media crates..."
    for crate in rustodon-media rustodon-storage; do
        if [ -d "$crate" ]; then
            mv "$crate" crates/media/
            print_success "Moved $crate to crates/media/"
        fi
    done

    # Federation crates
    print_status "Moving federation crates..."
    for crate in rustodon-federation rustodon-activitypub; do
        if [ -d "$crate" ]; then
            mv "$crate" crates/federation/
            print_success "Moved $crate to crates/federation/"
        fi
    done

    # Utility crates
    print_status "Moving utility crates..."
    for crate in rustodon-config rustodon-cache rustodon-queue rustodon-metrics rustodon-logging rustodon-mailer rustodon-search rustodon-scheduler rustodon-workers rustodon-webhooks; do
        if [ -d "$crate" ]; then
            mv "$crate" crates/utils/
            print_success "Moved $crate to crates/utils/"
        fi
    done

    # CLI crates
    print_status "Moving CLI crates..."
    for crate in rustodon-cli; do
        if [ -d "$crate" ]; then
            mv "$crate" crates/cli/
            print_success "Moved $crate to crates/cli/"
        fi
    done

    # Server crates
    print_status "Moving server crates..."
    for crate in rustodon-server rustodon-web; do
        if [ -d "$crate" ]; then
            mv "$crate" crates/server/
            print_success "Moved $crate to crates/server/"
        fi
    done

    print_success "All crates moved to organized structure"
}

# Update workspace Cargo.toml
update_workspace_toml() {
    print_status "Updating workspace Cargo.toml..."

    # Create new workspace Cargo.toml
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

# Core crates
members = [
    "crates/core/rustodon-core",
]

# API crates
members += [
    "crates/api/rustodon-api",
    "crates/api/rustodon-activitypub",
    "crates/api/rustodon-streaming",
]

# Auth crates
members += [
    "crates/auth/rustodon-auth",
    "crates/auth/rustodon-oauth",
    "crates/auth/rustodon-sessions",
    "crates/auth/rustodon-webauthn-credentials",
]

# Database crates
members += [
    "crates/database/rustodon-db",
    "crates/database/rustodon-migrations",
]

# Feature crates
members += [
    "crates/features/rustodon-accounts",
    "crates/features/rustodon-statuses",
    "crates/features/rustodon-notifications",
    "crates/features/rustodon-follows",
    "crates/features/rustodon-follow-requests",
    "crates/features/rustodon-favourites",
    "crates/features/rustodon-reblogs",
    "crates/features/rustodon-bookmarks",
    "crates/features/rustodon-lists",
    "crates/features/rustodon-conversations",
    "crates/features/rustodon-polls",
    "crates/features/rustodon-media",
    "crates/features/rustodon-tags",
    "crates/features/rustodon-mentions",
    "crates/features/rustodon-filters",
    "crates/features/rustodon-reports",
    "crates/features/rustodon-blocks",
    "crates/features/rustodon-mutes",
    "crates/features/rustodon-ip-blocks",
    "crates/features/rustodon-email-domain-blocks",
    "crates/features/rustodon-canonical-email-blocks",
    "crates/features/rustodon-domain-blocks",
    "crates/features/rustodon-custom-emojis",
    "crates/features/rustodon-trends",
    "crates/features/rustodon-announcements",
    "crates/features/rustodon-featured-tags",
    "crates/features/rustodon-endorsements",
    "crates/features/rustodon-suggestions",
    "crates/features/rustodon-directory",
    "crates/features/rustodon-instances",
    "crates/features/rustodon-peers",
    "crates/features/rustodon-activity",
    "crates/features/rustodon-annual-reports",
    "crates/features/rustodon-appeals",
    "crates/features/rustodon-bulk-imports",
    "crates/features/rustodon-encrypted-messages",
    "crates/features/rustodon-groups",
    "crates/features/rustodon-severed-relationships",
    "crates/features/rustodon-software-updates",
    "crates/features/rustodon-status-pins",
    "crates/features/rustodon-tag-follows",
    "crates/features/rustodon-terms-of-service",
    "crates/features/rustodon-user-settings",
    "crates/features/rustodon-webhooks",
    "crates/features/rustodon-workers",
    "crates/features/rustodon-scheduler",
    "crates/features/rustodon-queue",
    "crates/features/rustodon-cache",
    "crates/features/rustodon-storage",
    "crates/features/rustodon-search",
    "crates/features/rustodon-metrics",
    "crates/features/rustodon-logging",
    "crates/features/rustodon-mailer",
    "crates/features/rustodon-federation",
    "crates/features/rustodon-preview-cards",
    "crates/features/rustodon-scheduled-statuses",
    "crates/features/rustodon-access-grants",
    "crates/features/rustodon-access-tokens",
    "crates/features/rustodon-account-aliases",
    "crates/features/rustodon-account-conversations",
    "crates/features/rustodon-account-deletion-requests",
    "crates/features/rustodon-account-moderation-notes",
    "crates/features/rustodon-account-notes",
    "crates/features/rustodon-account-suggestions",
    "crates/features/rustodon-account-warnings",
    "crates/features/rustodon-analytics",
    "crates/features/rustodon-applications",
    "crates/features/rustodon-devices",
    "crates/features/rustodon-domains",
    "crates/features/rustodon-follow-recommendation-suppressions",
]

# Admin crates
members += [
    "crates/admin/rustodon-admin",
]

# Media crates
members += [
    "crates/media/rustodon-media",
    "crates/media/rustodon-storage",
]

# Federation crates
members += [
    "crates/federation/rustodon-federation",
    "crates/federation/rustodon-activitypub",
]

# Utility crates
members += [
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
]

# CLI crates
members += [
    "crates/cli/rustodon-cli",
]

# Server crates
members += [
    "crates/server/rustodon-server",
    "crates/server/rustodon-web",
]

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

    print_success "Workspace Cargo.toml updated"
}

# Create README for crates directory
create_crates_readme() {
    print_status "Creating crates directory README..."

    cat > crates/README.md << 'EOF'
# Rustodon Crates

This directory contains all the Rust crates that make up the Rustodon project, organized by functionality.

## Directory Structure

### Core (`core/`)
- **rustodon-core**: Core types, traits, and utilities shared across all crates

### API (`api/`)
- **rustodon-api**: HTTP API layer and endpoints
- **rustodon-activitypub**: ActivityPub protocol implementation
- **rustodon-streaming**: Real-time streaming functionality

### Authentication (`auth/`)
- **rustodon-auth**: Authentication and authorization
- **rustodon-oauth**: OAuth 2.0 implementation
- **rustodon-sessions**: Session management
- **rustodon-webauthn-credentials**: WebAuthn support

### Database (`database/`)
- **rustodon-db**: Database operations and models
- **rustodon-migrations**: Database migration management

### Features (`features/`)
All feature-specific crates including:
- Account management (accounts, follows, etc.)
- Content management (statuses, media, etc.)
- Social features (notifications, bookmarks, etc.)
- Moderation tools (reports, blocks, etc.)
- And many more...

### Admin (`admin/`)
- **rustodon-admin**: Administrative interface and tools

### Media (`media/`)
- **rustodon-media**: Media processing and management
- **rustodon-storage**: File storage abstraction

### Federation (`federation/`)
- **rustodon-federation**: Federation logic and protocols
- **rustodon-activitypub**: ActivityPub implementation

### Utilities (`utils/`)
- **rustodon-config**: Configuration management
- **rustodon-cache**: Caching layer
- **rustodon-queue**: Message queue
- **rustodon-metrics**: Metrics and monitoring
- **rustodon-logging**: Logging infrastructure
- **rustodon-mailer**: Email functionality
- **rustodon-search**: Search functionality
- **rustodon-scheduler**: Scheduled tasks
- **rustodon-workers**: Background job processing
- **rustodon-webhooks**: Webhook handling

### CLI (`cli/`)
- **rustodon-cli**: Command line interface

### Server (`server/`)
- **rustodon-server**: Main server binary
- **rustodon-web**: Web interface

## Development

To work on a specific crate:

```bash
# Navigate to the crate
cd crates/features/rustodon-accounts

# Build the crate
cargo build

# Run tests
cargo test

# Check for issues
cargo clippy
```

## Adding New Crates

When adding a new crate:

1. Create the crate in the appropriate subdirectory
2. Add it to the workspace members in the root `Cargo.toml`
3. Update this README with the new crate information
4. Ensure proper dependencies are declared

## Dependencies

Crates should depend on `rustodon-core` for shared types and utilities. Avoid circular dependencies between crates.
EOF

    print_success "Crates README created"
}

# Create organization summary
create_organization_summary() {
    print_status "Creating organization summary..."

    cat > PROJECT_ORGANIZATION.md << 'EOF'
# Rustodon Project Organization

## Overview

The Rustodon project has been reorganized to improve maintainability and structure. All Rust crates are now organized in the `crates/` directory with logical grouping.

## New Structure

```
rustodon/
├── Cargo.toml                 # Workspace configuration
├── crates/                    # All Rust crates
│   ├── core/                  # Core functionality
│   ├── api/                   # API layer
│   ├── auth/                  # Authentication
│   ├── database/              # Database operations
│   ├── features/              # Feature-specific crates
│   ├── admin/                 # Admin tools
│   ├── media/                 # Media handling
│   ├── federation/            # Federation logic
│   ├── utils/                 # Utilities
│   ├── cli/                   # Command line tools
│   └── server/                # Server binaries
├── docs/                      # Documentation
├── scripts/                   # Build and deployment scripts
├── tests/                     # Integration tests
├── migrations/                # Database migrations
├── public/                    # Static assets
├── storage/                   # File storage
└── streaming/                 # Streaming server
```

## Benefits

1. **Better Organization**: Related crates are grouped together
2. **Easier Navigation**: Clear directory structure
3. **Improved Maintainability**: Logical separation of concerns
4. **Better Scalability**: Easy to add new crates in appropriate locations
5. **Cleaner Root Directory**: Root directory is less cluttered

## Migration Notes

- All existing crates have been moved to appropriate subdirectories
- Workspace configuration has been updated
- Dependencies between crates remain the same
- Build and test commands work as before

## Development Workflow

The development workflow remains the same:

```bash
# Build all crates
cargo build

# Test all crates
cargo test

# Build specific crate
cargo build -p rustodon-accounts

# Test specific crate
cargo test -p rustodon-accounts
```

## Next Steps

1. Review the new structure
2. Update any hardcoded paths in scripts
3. Update documentation references
4. Consider further organization if needed
EOF

    print_success "Organization summary created"
}

# Main function
main() {
    print_status "Starting project organization..."

    # Check if we're in the right directory
    if [ ! -f "Cargo.toml" ]; then
        print_error "Not in Rustodon project root directory"
        exit 1
    fi

    # Create backup
    print_status "Creating backup of current structure..."
    cp Cargo.toml Cargo.toml.backup
    print_success "Backup created"

    # Execute organization steps
    create_crates_structure
    move_crates
    update_workspace_toml
    create_crates_readme
    create_organization_summary

    print_success "Project organization completed!"
    echo
    print_status "Summary:"
    echo "  - Created crates/ directory structure"
    echo "  - Moved all rustodon-* crates to organized subdirectories"
    echo "  - Updated workspace Cargo.toml"
    echo "  - Created documentation"
    echo
    print_status "Next steps:"
    echo "1. Review the new structure"
    echo "2. Test build: cargo check"
    echo "3. Commit changes: git add . && git commit -m 'refactor: organize crates into structured directory'"
    echo "4. Push to GitHub"
}

# Run main function
main
