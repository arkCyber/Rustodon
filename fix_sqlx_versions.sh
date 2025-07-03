#!/bin/bash

# Fix sqlx versions to be consistent across all crates
echo "Fixing sqlx versions to 0.7.3..."

# Find all Cargo.toml files that have sqlx dependency
find crates/ -name "Cargo.toml" -exec grep -l "sqlx" {} \; | while read -r file; do
    echo "Fixing sqlx version in $file"

    # Replace sqlx version to 0.7.3 with consistent features
    sed -i '' 's/sqlx = { version = "[^"]*"/sqlx = { version = "0.7.3"/g' "$file"
    sed -i '' 's/sqlx = "[^"]*"/sqlx = { version = "0.7.3", features = ["runtime-tokio-rustls", "postgres", "chrono", "uuid"] }/g' "$file"

    # Ensure consistent features for sqlx
    sed -i '' 's/sqlx = { version = "0.7.3"[^}]*}/sqlx = { version = "0.7.3", features = ["runtime-tokio-rustls", "postgres", "chrono", "uuid"] }/g' "$file"
done

echo "SQLx versions fixed!"
