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
