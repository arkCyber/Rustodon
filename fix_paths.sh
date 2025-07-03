#!/bin/bash

# Fix rustodon-core paths in all features crates
echo "Fixing rustodon-core paths in features crates..."

for file in crates/features/*/Cargo.toml; do
    if [ -f "$file" ]; then
        echo "Fixing $file"
        sed -i '' 's|rustodon-core = { path = "../rustodon-core"}|rustodon-core = { path = "../../core/rustodon-core"}|g' "$file"
    fi
done

echo "Path fixing completed!"
