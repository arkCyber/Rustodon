#!/bin/bash

# Simple Docker Test Script for Rustodon
# Author: arkSong (arksong2018@gmail.com)

set -e

echo "=== Rustodon Docker Test ==="
echo "Starting at: $(date)"

# Configuration
API_URL="http://localhost:3000"
DB_CONTAINER="rustodon-db"
SERVER_CONTAINER="rustodon-server"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Cleanup
print_status "Cleaning up existing containers..."
docker stop $DB_CONTAINER $SERVER_CONTAINER 2>/dev/null || true
docker rm $DB_CONTAINER $SERVER_CONTAINER 2>/dev/null || true

# Start database
print_status "Starting PostgreSQL..."
docker run -d \
    --name $DB_CONTAINER \
    -e POSTGRES_USER=rustodon \
    -e POSTGRES_PASSWORD=rustodon \
    -e POSTGRES_DB=rustodon \
    -p 5432:5432 \
    postgres:15-alpine

sleep 5

# Build server image
print_status "Building Rustodon server image..."
docker build -f rustodon/Dockerfile.simple -t rustodon:test rustodon/

# Start server
print_status "Starting Rustodon server..."
docker run -d \
    --name $SERVER_CONTAINER \
    --link $DB_CONTAINER:db \
    -e DATABASE_URL=postgres://rustodon:rustodon@db:5432/rustodon \
    -e RUST_LOG=info \
    -p 3000:3000 \
    rustodon:test

sleep 10

# Test API
print_status "Testing API endpoints..."

# Health check
echo "1. Health check..."
curl -s "$API_URL/health" || echo "Health check failed"

# API version
echo "2. API version..."
curl -s "$API_URL/api/v1/instance" || echo "API version check failed"

# Registration
echo "3. User registration..."
curl -s -X POST "$API_URL/api/v1/auth/register" \
    -H "Content-Type: application/json" \
    -d '{"username":"testuser","email":"test@example.com","password":"testpass","agreement":true,"locale":"en"}' || echo "Registration failed"

print_success "Docker test completed!"
