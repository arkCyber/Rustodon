#!/bin/bash
set -e

echo "Fixing axum version from 0.7 to 0.6 across all crates..."

# Find all Cargo.toml files in sub-crates
find . -name "Cargo.toml" -not -path "./Cargo.toml" | while read -r file; do
    echo "Processing: $file"

    # Check if the file has axum dependency
    if grep -q "axum" "$file"; then
        # Update axum version from 0.7 to 0.6
        sed -i '' 's/axum = { version = "0\.7"/axum = { version = "0.6"/g' "$file"
        sed -i '' 's/axum = "0\.7"/axum = "0.6"/g' "$file"

        echo "  Updated axum version in $file"
    else
        echo "  No axum dependency found in $file"
    fi
done

echo "Axum version fixed!"
