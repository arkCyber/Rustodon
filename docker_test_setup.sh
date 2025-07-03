#!/bin/bash

# Docker Test Setup Script for Rustodon
# Author: arkSong (arksong2018@gmail.com)
# Project: rustodon
#
# This script sets up a complete Docker environment for testing Rustodon
# and runs comprehensive curl tests against the API

set -e

echo "=== Rustodon Docker Test Setup ==="
echo "Starting at: $(date)"
echo

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
API_URL="http://localhost:3000"
DB_CONTAINER="rustodon-db"
SERVER_CONTAINER="rustodon-server"
NETWORK_NAME="rustodon-test"

# Function to print colored output
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

# Function to wait for service to be ready
wait_for_service() {
    local url=$1
    local service_name=$2
    local max_attempts=30
    local attempt=1

    print_status "Waiting for $service_name to be ready..."

    while [ $attempt -le $max_attempts ]; do
        if curl -s -f "$url" > /dev/null 2>&1; then
            print_success "$service_name is ready!"
            return 0
        fi

        print_status "Attempt $attempt/$max_attempts - $service_name not ready yet..."
        sleep 2
        attempt=$((attempt + 1))
    done

    print_error "$service_name failed to start within $((max_attempts * 2)) seconds"
    return 1
}

# Function to check if Docker is running
check_docker() {
    if ! docker info > /dev/null 2>&1; then
        print_error "Docker is not running. Please start Docker and try again."
        exit 1
    fi
    print_success "Docker is running"
}

# Function to clean up existing containers
cleanup() {
    print_status "Cleaning up existing containers..."

    # Stop and remove containers
    docker stop $DB_CONTAINER $SERVER_CONTAINER 2>/dev/null || true
    docker rm $DB_CONTAINER $SERVER_CONTAINER 2>/dev/null || true

    # Remove network
    docker network rm $NETWORK_NAME 2>/dev/null || true

    print_success "Cleanup completed"
}

# Function to create Docker network
create_network() {
    print_status "Creating Docker network..."
    docker network create $NETWORK_NAME 2>/dev/null || true
    print_success "Network created"
}

# Function to start database
start_database() {
    print_status "Starting PostgreSQL database..."

    docker run -d \
        --name $DB_CONTAINER \
        --network $NETWORK_NAME \
        -e POSTGRES_USER=rustodon \
        -e POSTGRES_PASSWORD=rustodon \
        -e POSTGRES_DB=rustodon \
        -p 5432:5432 \
        postgres:15-alpine

    # Wait for database to be ready
    sleep 5
    wait_for_service "http://localhost:5432" "PostgreSQL"
}

# Function to start Rustodon server
start_server() {
    print_status "Starting Rustodon server..."

    # Build the image if it doesn't exist
    if ! docker images | grep -q "rustodon:test"; then
        print_status "Building Rustodon Docker image..."
        docker build -f rustodon/Dockerfile.simple -t rustodon:test .
    fi

    # Start the server
    docker run -d \
        --name $SERVER_CONTAINER \
        --network $NETWORK_NAME \
        -e DATABASE_URL=postgres://rustodon:rustodon@$DB_CONTAINER:5432/rustodon \
        -e RUST_LOG=info \
        -p 3000:3000 \
        rustodon:test

    # Wait for server to be ready
    wait_for_service "$API_URL/health" "Rustodon Server"
}

# Function to run comprehensive API tests
run_api_tests() {
    print_status "Running comprehensive API tests..."

    # Create test script
    cat > api_test.sh << 'EOF'
#!/bin/bash

API_URL="http://localhost:3000"
TEST_USER="testuser"
TEST_EMAIL="test@example.com"
TEST_PASSWORD="testpass123"

echo "=== API Test Results ==="
echo "Timestamp: $(date)"
echo

# Test 1: Health Check
echo "1. Testing Health Check..."
HEALTH_RESPONSE=$(curl -s -w "%{http_code}" "$API_URL/health" -o /tmp/health_response)
if [ "$HEALTH_RESPONSE" = "200" ]; then
    echo "   ✓ Health check passed"
    cat /tmp/health_response
else
    echo "   ✗ Health check failed (HTTP $HEALTH_RESPONSE)"
fi
echo

# Test 2: API Version
echo "2. Testing API Version..."
VERSION_RESPONSE=$(curl -s -w "%{http_code}" "$API_URL/api/v1/instance" -o /tmp/version_response)
if [ "$VERSION_RESPONSE" = "200" ]; then
    echo "   ✓ API version check passed"
    cat /tmp/version_response
else
    echo "   ✗ API version check failed (HTTP $VERSION_RESPONSE)"
fi
echo

# Test 3: User Registration
echo "3. Testing User Registration..."
REGISTER_RESPONSE=$(curl -s -w "%{http_code}" \
    -X POST "$API_URL/api/v1/auth/register" \
    -H "Content-Type: application/json" \
    -d "{\"username\":\"$TEST_USER\",\"email\":\"$TEST_EMAIL\",\"password\":\"$TEST_PASSWORD\",\"agreement\":true,\"locale\":\"en\"}" \
    -o /tmp/register_response)
if [ "$REGISTER_RESPONSE" = "200" ] || [ "$REGISTER_RESPONSE" = "422" ]; then
    echo "   ✓ Registration endpoint responded (HTTP $REGISTER_RESPONSE)"
    cat /tmp/register_response
else
    echo "   ✗ Registration failed (HTTP $REGISTER_RESPONSE)"
fi
echo

# Test 4: User Login
echo "4. Testing User Login..."
LOGIN_RESPONSE=$(curl -s -w "%{http_code}" \
    -X POST "$API_URL/api/v1/auth/login" \
    -H "Content-Type: application/json" \
    -d "{\"username_or_email\":\"$TEST_USER\",\"password\":\"$TEST_PASSWORD\"}" \
    -o /tmp/login_response)
if [ "$LOGIN_RESPONSE" = "200" ] || [ "$LOGIN_RESPONSE" = "401" ]; then
    echo "   ✓ Login endpoint responded (HTTP $LOGIN_RESPONSE)"
    cat /tmp/login_response
else
    echo "   ✗ Login failed (HTTP $LOGIN_RESPONSE)"
fi
echo

# Test 5: Status Creation (if we have a token)
echo "5. Testing Status Creation..."
TOKEN=$(cat /tmp/login_response | grep -o '"token":"[^"]*"' | cut -d'"' -f4)
if [ -n "$TOKEN" ]; then
    STATUS_RESPONSE=$(curl -s -w "%{http_code}" \
        -X POST "$API_URL/api/v1/statuses" \
        -H "Authorization: Bearer $TOKEN" \
        -H "Content-Type: application/json" \
        -d '{"status":"Hello from Docker test! #rustodon"}' \
        -o /tmp/status_response)
    if [ "$STATUS_RESPONSE" = "200" ]; then
        echo "   ✓ Status creation passed"
        cat /tmp/status_response
    else
        echo "   ✗ Status creation failed (HTTP $STATUS_RESPONSE)"
    fi
else
    echo "   ⚠ No token available for status creation test"
fi
echo

# Test 6: Timeline Endpoints
echo "6. Testing Timeline Endpoints..."
TIMELINE_RESPONSE=$(curl -s -w "%{http_code}" "$API_URL/api/v1/timelines/public" -o /tmp/timeline_response)
if [ "$TIMELINE_RESPONSE" = "200" ]; then
    echo "   ✓ Public timeline endpoint passed"
else
    echo "   ✗ Public timeline failed (HTTP $TIMELINE_RESPONSE)"
fi
echo

# Test 7: Account Endpoints
echo "7. Testing Account Endpoints..."
ACCOUNT_RESPONSE=$(curl -s -w "%{http_code}" "$API_URL/api/v1/accounts/1" -o /tmp/account_response)
if [ "$ACCOUNT_RESPONSE" = "200" ] || [ "$ACCOUNT_RESPONSE" = "404" ]; then
    echo "   ✓ Account endpoint responded (HTTP $ACCOUNT_RESPONSE)"
else
    echo "   ✗ Account endpoint failed (HTTP $ACCOUNT_RESPONSE)"
fi
echo

# Test 8: Search Endpoint
echo "8. Testing Search Endpoint..."
SEARCH_RESPONSE=$(curl -s -w "%{http_code}" "$API_URL/api/v1/search?q=test" -o /tmp/search_response)
if [ "$SEARCH_RESPONSE" = "200" ] || [ "$SEARCH_RESPONSE" = "401" ]; then
    echo "   ✓ Search endpoint responded (HTTP $SEARCH_RESPONSE)"
else
    echo "   ✗ Search endpoint failed (HTTP $SEARCH_RESPONSE)"
fi
echo

# Test 9: Media Upload (if we have a token)
echo "9. Testing Media Upload..."
if [ -n "$TOKEN" ]; then
    # Create a test image file
    echo "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==" | base64 -d > /tmp/test.png

    MEDIA_RESPONSE=$(curl -s -w "%{http_code}" \
        -X POST "$API_URL/api/v1/media" \
        -H "Authorization: Bearer $TOKEN" \
        -F "file=@/tmp/test.png" \
        -o /tmp/media_response)
    if [ "$MEDIA_RESPONSE" = "200" ]; then
        echo "   ✓ Media upload passed"
        cat /tmp/media_response
    else
        echo "   ✗ Media upload failed (HTTP $MEDIA_RESPONSE)"
    fi
else
    echo "   ⚠ No token available for media upload test"
fi
echo

# Test 10: Notifications (if we have a token)
echo "10. Testing Notifications..."
if [ -n "$TOKEN" ]; then
    NOTIFICATIONS_RESPONSE=$(curl -s -w "%{http_code}" \
        -X GET "$API_URL/api/v1/notifications" \
        -H "Authorization: Bearer $TOKEN" \
        -o /tmp/notifications_response)
    if [ "$NOTIFICATIONS_RESPONSE" = "200" ]; then
        echo "   ✓ Notifications endpoint passed"
        cat /tmp/notifications_response
    else
        echo "   ✗ Notifications failed (HTTP $NOTIFICATIONS_RESPONSE)"
    fi
else
    echo "   ⚠ No token available for notifications test"
fi
echo

# Cleanup
rm -f /tmp/*_response /tmp/test.png

echo "=== API Test Summary ==="
echo "Tests completed at: $(date)"
echo "Server URL: $API_URL"
EOF

    chmod +x api_test.sh

    # Run the test script
    ./api_test.sh

    print_success "API tests completed"
}

# Function to show container logs
show_logs() {
    print_status "Showing container logs..."
    echo
    echo "=== Database Logs ==="
    docker logs $DB_CONTAINER --tail 20
    echo
    echo "=== Server Logs ==="
    docker logs $SERVER_CONTAINER --tail 20
}

# Function to show container status
show_status() {
    print_status "Container Status:"
    docker ps -a --filter "name=rustodon"
    echo
    print_status "Network Status:"
    docker network ls --filter "name=rustodon"
}

# Main execution
main() {
    print_status "Starting Rustodon Docker test setup..."

    # Check prerequisites
    check_docker

    # Cleanup and setup
    cleanup
    create_network

    # Start services
    start_database
    start_server

    # Show status
    show_status

    # Run tests
    run_api_tests

    # Show logs
    show_logs

    print_success "Docker test setup completed!"
    echo
    print_status "Services are running:"
    echo "  - Database: localhost:5432"
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
