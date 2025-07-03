#!/bin/bash

# Comprehensive Curl Test Script for Rustodon
# Author: arkSong (arksong2018@gmail.com)
# Project: rustodon
#
# This script performs comprehensive curl tests against the Rustodon API
# It tests all major endpoints and provides detailed results

set -e

echo "=== Rustodon Comprehensive Curl Test ==="
echo "Starting at: $(date)"
echo

# Configuration
API_URL="http://localhost:3000"
TEST_USER="testuser"
TEST_EMAIL="test@example.com"
TEST_PASSWORD="testpass123"

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

# Function to test an endpoint
test_endpoint() {
    local method=$1
    local endpoint=$2
    local data=$3
    local description=$4

    echo "Testing: $description"
    echo "Endpoint: $method $endpoint"

    local response
    if [ "$method" = "GET" ]; then
        response=$(curl -s -w "%{http_code}" "$API_URL$endpoint" -o /tmp/response.json)
    elif [ "$method" = "POST" ]; then
        response=$(curl -s -w "%{http_code}" -X POST "$API_URL$endpoint" \
            -H "Content-Type: application/json" \
            -d "$data" \
            -o /tmp/response.json)
    fi

    echo "HTTP Status: $response"

    if [ -f /tmp/response.json ]; then
        echo "Response:"
        cat /tmp/response.json | head -c 200
        echo "..."
    fi

    if [ "$response" = "200" ] || [ "$response" = "201" ]; then
        print_success "✓ $description passed"
    elif [ "$response" = "401" ] || [ "$response" = "404" ] || [ "$response" = "422" ]; then
        print_warning "⚠ $description responded with $response (expected for some endpoints)"
    else
        print_error "✗ $description failed with $response"
    fi

    echo
}

# Function to test authenticated endpoint
test_auth_endpoint() {
    local method=$1
    local endpoint=$2
    local data=$3
    local description=$4
    local token=$5

    echo "Testing: $description"
    echo "Endpoint: $method $endpoint"

    local response
    if [ "$method" = "GET" ]; then
        response=$(curl -s -w "%{http_code}" \
            -H "Authorization: Bearer $token" \
            "$API_URL$endpoint" \
            -o /tmp/response.json)
    elif [ "$method" = "POST" ]; then
        response=$(curl -s -w "%{http_code}" \
            -X POST "$API_URL$endpoint" \
            -H "Authorization: Bearer $token" \
            -H "Content-Type: application/json" \
            -d "$data" \
            -o /tmp/response.json)
    fi

    echo "HTTP Status: $response"

    if [ -f /tmp/response.json ]; then
        echo "Response:"
        cat /tmp/response.json | head -c 200
        echo "..."
    fi

    if [ "$response" = "200" ] || [ "$response" = "201" ]; then
        print_success "✓ $description passed"
    else
        print_error "✗ $description failed with $response"
    fi

    echo
}

# Main test execution
main() {
    print_status "Starting comprehensive API tests..."
    echo

    # Test 1: Health Check
    test_endpoint "GET" "/health" "" "Health Check"

    # Test 2: API Instance Info
    test_endpoint "GET" "/api/v1/instance" "" "API Instance Information"

    # Test 3: User Registration
    test_endpoint "POST" "/api/v1/auth/register" \
        "{\"username\":\"$TEST_USER\",\"email\":\"$TEST_EMAIL\",\"password\":\"$TEST_PASSWORD\",\"agreement\":true,\"locale\":\"en\"}" \
        "User Registration"

    # Test 4: User Login
    test_endpoint "POST" "/api/v1/auth/login" \
        "{\"username_or_email\":\"$TEST_USER\",\"password\":\"$TEST_PASSWORD\"}" \
        "User Login"

    # Extract token from login response
    TOKEN=""
    if [ -f /tmp/response.json ]; then
        TOKEN=$(cat /tmp/response.json | grep -o '"token":"[^"]*"' | cut -d'"' -f4 || echo "")
    fi

    # Test 5: Public Timeline
    test_endpoint "GET" "/api/v1/timelines/public" "" "Public Timeline"

    # Test 6: Account Information
    test_endpoint "GET" "/api/v1/accounts/1" "" "Account Information"

    # Test 7: Search
    test_endpoint "GET" "/api/v1/search?q=test" "" "Search Functionality"

    # Test 8: Status Creation (if we have a token)
    if [ -n "$TOKEN" ]; then
        test_auth_endpoint "POST" "/api/v1/statuses" \
            "{\"status\":\"Hello from comprehensive test! #rustodon\"}" \
            "Status Creation" \
            "$TOKEN"
    else
        print_warning "No token available for authenticated tests"
    fi

    # Test 9: Media Upload (if we have a token)
    if [ -n "$TOKEN" ]; then
        echo "Testing: Media Upload"
        echo "Endpoint: POST /api/v1/media"

        # Create a test image
        echo "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==" | base64 -d > /tmp/test.png

        response=$(curl -s -w "%{http_code}" \
            -X POST "$API_URL/api/v1/media" \
            -H "Authorization: Bearer $TOKEN" \
            -F "file=@/tmp/test.png" \
            -o /tmp/response.json)

        echo "HTTP Status: $response"
        if [ -f /tmp/response.json ]; then
            echo "Response:"
            cat /tmp/response.json | head -c 200
            echo "..."
        fi

        if [ "$response" = "200" ]; then
            print_success "✓ Media Upload passed"
        else
            print_error "✗ Media Upload failed with $response"
        fi
        echo
    fi

    # Test 10: Notifications (if we have a token)
    if [ -n "$TOKEN" ]; then
        test_auth_endpoint "GET" "/api/v1/notifications" "" "Notifications" "$TOKEN"
    fi

    # Test 11: Follow/Unfollow (if we have a token)
    if [ -n "$TOKEN" ]; then
        test_auth_endpoint "POST" "/api/v1/accounts/1/follow" "" "Follow User" "$TOKEN"
        test_auth_endpoint "POST" "/api/v1/accounts/1/unfollow" "" "Unfollow User" "$TOKEN"
    fi

    # Test 12: Favourite/Unfavourite (if we have a token)
    if [ -n "$TOKEN" ]; then
        test_auth_endpoint "POST" "/api/v1/statuses/1/favourite" "" "Favourite Status" "$TOKEN"
        test_auth_endpoint "POST" "/api/v1/statuses/1/unfavourite" "" "Unfavourite Status" "$TOKEN"
    fi

    # Test 13: Reblog/Unreblog (if we have a token)
    if [ -n "$TOKEN" ]; then
        test_auth_endpoint "POST" "/api/v1/statuses/1/reblog" "" "Reblog Status" "$TOKEN"
        test_auth_endpoint "POST" "/api/v1/statuses/1/unreblog" "" "Unreblog Status" "$TOKEN"
    fi

    # Test 14: Lists
    if [ -n "$TOKEN" ]; then
        test_auth_endpoint "GET" "/api/v1/lists" "" "Get Lists" "$TOKEN"
        test_auth_endpoint "POST" "/api/v1/lists" \
            "{\"title\":\"Test List\"}" \
            "Create List" \
            "$TOKEN"
    fi

    # Test 15: Conversations
    if [ -n "$TOKEN" ]; then
        test_auth_endpoint "GET" "/api/v1/conversations" "" "Get Conversations" "$TOKEN"
    fi

    # Test 16: Bookmarks
    if [ -n "$TOKEN" ]; then
        test_auth_endpoint "GET" "/api/v1/bookmarks" "" "Get Bookmarks" "$TOKEN"
    fi

    # Test 17: Mutes
    if [ -n "$TOKEN" ]; then
        test_auth_endpoint "GET" "/api/v1/mutes" "" "Get Mutes" "$TOKEN"
    fi

    # Test 18: Blocks
    if [ -n "$TOKEN" ]; then
        test_auth_endpoint "GET" "/api/v1/blocks" "" "Get Blocks" "$TOKEN"
    fi

    # Test 19: Reports
    if [ -n "$TOKEN" ]; then
        test_auth_endpoint "GET" "/api/v1/reports" "" "Get Reports" "$TOKEN"
    fi

    # Test 20: Filters
    if [ -n "$TOKEN" ]; then
        test_auth_endpoint "GET" "/api/v1/filters" "" "Get Filters" "$TOKEN"
    fi

    # Cleanup
    rm -f /tmp/response.json /tmp/test.png

    print_success "Comprehensive API tests completed!"
    echo
    print_status "Test Summary:"
    echo "  - API URL: $API_URL"
    echo "  - Test User: $TEST_USER"
    echo "  - Authentication: $([ -n "$TOKEN" ] && echo "Available" || echo "Not available")"
    echo
    print_status "To run individual tests, use:"
    echo "  curl -s $API_URL/health"
    echo "  curl -s $API_URL/api/v1/instance"
    echo "  curl -s -X POST $API_URL/api/v1/auth/register -H 'Content-Type: application/json' -d '{\"username\":\"test\",\"email\":\"test@example.com\",\"password\":\"testpass\",\"agreement\":true,\"locale\":\"en\"}'"
}

# Run main function
main "$@"
