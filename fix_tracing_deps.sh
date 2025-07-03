#!/bin/bash
set -e

echo "Fixing tracing dependencies across all crates..."

# Find all Cargo.toml files in sub-crates
find . -name "Cargo.toml" -not -path "./Cargo.toml" | while read -r file; do
    echo "Processing: $file"

    # Check if the file already has tracing dependency
    if ! grep -q "tracing = { workspace = true }" "$file"; then
        # Check if it has any tracing dependency
        if grep -q "tracing" "$file"; then
            # Remove existing tracing lines
            sed -i '' '/tracing =/d' "$file"
        fi

        # Add tracing = { workspace = true } after [dependencies]
        sed -i '' '/\[dependencies\]/a\
tracing = { workspace = true }' "$file"

        echo "  Added tracing = { workspace = true } to $file"
    else
        echo "  Already has correct tracing dependency: $file"
    fi
done

echo "Tracing dependencies fixed!"
