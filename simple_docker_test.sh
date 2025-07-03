#!/bin/bash

# Simple Docker Test Script for Rustodon
# Author: arkSong (arksong2018@gmail.com)
# Project: rustodon
#
# This script provides a simplified Docker test environment
# focusing on core API functionality testing

set -e

echo "=== Rustodon Simple Docker Test ==="
echo "Starting at: $(date)"
echo

# Configuration
API_URL="http://localhost:3000"
DB_CONTAINER="rustodon-db-simple"
SERVER_CONTAINER="rustodon-server-simple"
NETWORK_NAME="rustodon-simple"

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

# Function to wait for service
wait_for_service() {
    local url=$1
    local service_name=$2
    local max_attempts=15
    local attempt=1

    print_status "Waiting for $service_name to be ready..."

    while [ $attempt -le $max_attempts ]; do
        if curl -s -f "$url" > /dev/null 2>&1; then
            print_success "$service_name is ready!"
            return 0
        fi

        print_status "Attempt $attempt/$max_attempts - $service_name not ready yet..."
        sleep 3
        attempt=$((attempt + 1))
    done

    print_error "$service_name failed to start within $((max_attempts * 3)) seconds"
    return 1
}

# Cleanup function
cleanup() {
    print_status "Cleaning up containers..."
    docker stop $DB_CONTAINER $SERVER_CONTAINER 2>/dev/null || true
    docker rm $DB_CONTAINER $SERVER_CONTAINER 2>/dev/null || true
    docker network rm $NETWORK_NAME 2>/dev/null || true
    print_success "Cleanup completed"
}

# Create network
create_network() {
    print_status "Creating Docker network..."
    docker network create $NETWORK_NAME 2>/dev/null || true
    print_success "Network created"
}

# Start database
start_database() {
    print_status "Starting PostgreSQL database..."

    docker run -d \
        --name $DB_CONTAINER \
        --network $NETWORK_NAME \
        -e POSTGRES_USER=rustodon \
        -e POSTGRES_PASSWORD=rustodon \
        -e POSTGRES_DB=rustodon \
        -p 5433:5432 \
        postgres:15-alpine

    sleep 5
    print_success "Database started"
}

# Build and start server
start_server() {
    print_status "Building minimal test server..."

    # Build the minimal test image
    docker build -f rustodon/Dockerfile.minimal -t rustodon:test .

    print_status "Starting test server..."

    # Start the test server
    docker run -d \
        --name $SERVER_CONTAINER \
        --network $NETWORK_NAME \
        -p 3000:3000 \
        rustodon:test

    sleep 5
    print_success "Test server started"
}

# Run API tests
run_api_tests() {
    print_status "Running API tests..."
    echo

    # Test 1: Health Check
    echo "1. Testing Health Check..."
    if curl -s "$API_URL/health" > /dev/null; then
        print_success "Health check passed"
        curl -s "$API_URL/health" | head -c 100
        echo "..."
    else
        print_error "Health check failed"
    fi
    echo

    # Test 2: API Version
    echo "2. Testing API Version..."
    if curl -s "$API_URL/api/v1/instance" > /dev/null; then
        print_success "API version check passed"
        curl -s "$API_URL/api/v1/instance" | head -c 100
        echo "..."
    else
        print_error "API version check failed"
    fi
    echo

    # Test 3: User Registration
    echo "3. Testing User Registration..."
    REGISTER_RESPONSE=$(curl -s -w "%{http_code}" \
        -X POST "$API_URL/api/v1/auth/register" \
        -H "Content-Type: application/json" \
        -d '{"username":"testuser","email":"test@example.com","password":"testpass123","agreement":true,"locale":"en"}' \
        -o /tmp/register_response)

    if [ "$REGISTER_RESPONSE" = "200" ] || [ "$REGISTER_RESPONSE" = "422" ]; then
        print_success "Registration endpoint responded (HTTP $REGISTER_RESPONSE)"
        cat /tmp/register_response
    else
        print_error "Registration failed (HTTP $REGISTER_RESPONSE)"
    fi
    echo

    # Test 4: User Login
    echo "4. Testing User Login..."
    LOGIN_RESPONSE=$(curl -s -w "%{http_code}" \
        -X POST "$API_URL/api/v1/auth/login" \
        -H "Content-Type: application/json" \
        -d '{"username_or_email":"testuser","password":"testpass123"}' \
        -o /tmp/login_response)

    if [ "$LOGIN_RESPONSE" = "200" ] || [ "$LOGIN_RESPONSE" = "401" ]; then
        print_success "Login endpoint responded (HTTP $LOGIN_RESPONSE)"
        cat /tmp/login_response
    else
        print_error "Login failed (HTTP $LOGIN_RESPONSE)"
    fi
    echo

    # Test 5: Status Creation (if we have a token)
    echo "5. Testing Status Creation..."
    TOKEN=$(cat /tmp/login_response 2>/dev/null | grep -o '"token":"[^"]*"' | cut -d'"' -f4 || echo "")
    if [ -n "$TOKEN" ]; then
        STATUS_RESPONSE=$(curl -s -w "%{http_code}" \
            -X POST "$API_URL/api/v1/statuses" \
            -H "Authorization: Bearer $TOKEN" \
            -H "Content-Type: application/json" \
            -d '{"status":"Hello from Docker test! #rustodon"}' \
            -o /tmp/status_response)

        if [ "$STATUS_RESPONSE" = "200" ]; then
            print_success "Status creation passed"
            cat /tmp/status_response
        else
            print_error "Status creation failed (HTTP $STATUS_RESPONSE)"
        fi
    else
        print_warning "No token available for status creation test"
    fi
    echo

    # Test 6: Timeline Endpoints
    echo "6. Testing Timeline Endpoints..."
    TIMELINE_RESPONSE=$(curl -s -w "%{http_code}" "$API_URL/api/v1/timelines/public" -o /tmp/timeline_response)
    if [ "$TIMELINE_RESPONSE" = "200" ]; then
        print_success "Public timeline endpoint passed"
        cat /tmp/timeline_response | head -c 100
        echo "..."
    else
        print_error "Public timeline failed (HTTP $TIMELINE_RESPONSE)"
    fi
    echo

    # Test 7: Account Endpoints
    echo "7. Testing Account Endpoints..."
    ACCOUNT_RESPONSE=$(curl -s -w "%{http_code}" "$API_URL/api/v1/accounts/1" -o /tmp/account_response)
    if [ "$ACCOUNT_RESPONSE" = "200" ] || [ "$ACCOUNT_RESPONSE" = "404" ]; then
        print_success "Account endpoint responded (HTTP $ACCOUNT_RESPONSE)"
        cat /tmp/account_response | head -c 100
        echo "..."
    else
        print_error "Account endpoint failed (HTTP $ACCOUNT_RESPONSE)"
    fi
    echo

    # Test 8: Search Endpoint
    echo "8. Testing Search Endpoint..."
    SEARCH_RESPONSE=$(curl -s -w "%{http_code}" "$API_URL/api/v1/search?q=test" -o /tmp/search_response)
    if [ "$SEARCH_RESPONSE" = "200" ] || [ "$SEARCH_RESPONSE" = "401" ]; then
        print_success "Search endpoint responded (HTTP $SEARCH_RESPONSE)"
        cat /tmp/search_response | head -c 100
        echo "..."
    else
        print_error "Search endpoint failed (HTTP $SEARCH_RESPONSE)"
    fi
    echo

    # Cleanup temp files
    rm -f /tmp/*_response

    print_success "API tests completed"
}

# Show container status
show_status() {
    print_status "Container Status:"
    docker ps -a --filter "name=rustodon"
    echo
    print_status "Network Status:"
    docker network ls --filter "name=rustodon"
}

# Show logs
show_logs() {
    print_status "Container Logs:"
    echo
    echo "=== Database Logs ==="
    docker logs $DB_CONTAINER --tail 10 2>/dev/null || echo "No database logs available"
    echo
    echo "=== Server Logs ==="
    docker logs $SERVER_CONTAINER --tail 10 2>/dev/null || echo "No server logs available"
}

# Main execution
main() {
    print_status "Starting simplified Rustodon Docker test..."

    # Check if Docker is running
    if ! docker info > /dev/null 2>&1; then
        print_error "Docker is not running. Please start Docker and try again."
        exit 1
    fi

    # Setup environment
    cleanup
    create_network
    start_database

    # Start test server
    start_server
    wait_for_service "$API_URL" "Test Server" || true

    # Show status
    show_status

    # Run tests
    run_api_tests

    # Show logs
    show_logs

    print_success "Simplified Docker test completed!"
    echo
    print_status "Services status:"
    echo "  - Database: localhost:5433"
    echo "  - Server: $API_URL"
    echo
    print_status "To stop services, run:"
    echo "  docker stop $DB_CONTAINER $SERVER_CONTAINER"
    echo
    print_status "To view logs, run:"
    echo "  docker logs $SERVER_CONTAINER -f"
}

# Handle script interruption
trap cleanup EXIT

# Run main function
main "$@"
