#!/bin/bash

echo "Testing compilation without database connection..."

# Create a temporary backup
cp rustodon-db/src/models/oauth_access_token.rs rustodon-db/src/models/oauth_access_token.rs.bak
cp rustodon-db/src/models/list.rs rustodon-db/src/models/list.rs.bak

# Temporarily rename files to disable them
mv rustodon-db/src/models/oauth_access_token.rs rustodon-db/src/models/oauth_access_token.rs.disabled
mv rustodon-db/src/models/list.rs rustodon-db/src/models/list.rs.disabled

# Test compilation
cargo check -p rustodon-db

# Restore files
mv rustodon-db/src/models/oauth_access_token.rs.disabled rustodon-db/src/models/oauth_access_token.rs
mv rustodon-db/src/models/list.rs.disabled rustodon-db/src/models/list.rs

echo "Compilation test completed."
