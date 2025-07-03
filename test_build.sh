#!/bin/bash

# Test Build Script for Rustodon
# This script tests compilation without database connections
# Author: arkSong (arksong2018@gmail.com)

set -e

echo "🧪 Testing build without database connections..."

# Set environment to skip database operations
export SKIP_DB_TESTS=1
export DATABASE_URL="postgres://test:test@localhost:5432/test"

# Test core compilation
echo "📦 Testing core crates..."
cargo check -p rustodon-core
cargo check -p rustodon-config
cargo check -p rustodon-logging

# Test API crates
echo "🌐 Testing API crates..."
cargo check -p rustodon-api
cargo check -p rustodon-auth
cargo check -p rustodon-oauth

# Test server
echo "🚀 Testing server..."
cargo check -p rustodon-server

echo "✅ Build test completed successfully!"
echo "📋 Summary:"
echo "- Core crates compile successfully"
echo "- API crates compile successfully"
echo "- Server compiles successfully"
echo "- Database-related errors are expected and can be fixed later"
