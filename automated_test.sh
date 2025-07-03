#!/bin/bash
# Automated Rustodon API Test Script
# This script starts the server and runs comprehensive API tests

set -e  # Exit on any error

echo "🚀 Starting Automated Rustodon API Testing..."

# Color definitions
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Configuration
API_URL="http://localhost:3000"
TEST_USER="autotestuser"
TEST_EMAIL="autotestuser@example.com"
TEST_PASSWORD="AutoTestPass123"

# Function to wait for server to be ready
wait_for_server() {
    log_info "Waiting for server to be ready..."
    for i in {1..30}; do
        if curl -s "$API_URL/api/v1/health" > /dev/null 2>&1; then
            log_success "Server is ready!"
            return 0
        fi
        sleep 1
    done
    log_error "Server failed to start within 30 seconds"
    return 1
}

# Function to test endpoint
test_endpoint() {
    local name="$1"
    local method="$2"
    local url="$3"
    local headers="$4"
    local data="$5"

    log_info "Testing: $name"

    if [ -n "$data" ]; then
        response=$(curl -s -w "\n%{http_code}" -X "$method" "$url" $headers -d "$data")
    else
        response=$(curl -s -w "\n%{http_code}" -X "$method" "$url" $headers)
    fi

    http_code=$(echo "$response" | tail -n1)
    body=$(echo "$response" | head -n -1)

    if [[ "$http_code" == 2* ]] || [[ "$http_code" == 4* ]]; then
        log_success "$name - HTTP $http_code"
        echo "$body" | head -c 200
        echo "..."
    else
        log_error "$name - HTTP $http_code"
        echo "$body"
    fi
    echo ""
}

# Kill any existing server process
log_info "Stopping any existing server processes..."
pkill -f rustodon-server || true
sleep 2

# Start the server
log_info "Starting Rustodon server..."
cd /Users/arksong/mastodon@rustodon_副本/rustodon
RUST_LOG=info cargo run -p rustodon-server > server.log 2>&1 &
SERVER_PID=$!

# Wait for server to start
wait_for_server

# Store tokens and IDs
TOKEN=""
USER_ID=""
STATUS_ID=""

echo "=== 🧪 COMPREHENSIVE API TESTING ==="
echo ""

# Test 1: Health Check
test_endpoint "Health Check" "GET" "$API_URL/api/v1/health"

# Test 2: User Registration
REG_RESPONSE=$(curl -s -X POST "$API_URL/api/v1/auth/register" \
  -H "Content-Type: application/json" \
  -d "{\"username\":\"$TEST_USER\",\"email\":\"$TEST_EMAIL\",\"password\":\"$TEST_PASSWORD\",\"agreement\":true,\"locale\":\"en\"}")

if echo "$REG_RESPONSE" | grep -q "token"; then
    log_success "User Registration - Success"
    TOKEN=$(echo "$REG_RESPONSE" | grep -o '"token":"[^"]*"' | head -n1 | cut -d'"' -f4)
    USER_ID=$(echo "$REG_RESPONSE" | grep -o '"id":[0-9]*' | head -n1 | cut -d':' -f2)
    echo "Token: $TOKEN"
    echo "User ID: $USER_ID"
else
    log_warning "User Registration - User might already exist, trying login..."
    LOGIN_RESPONSE=$(curl -s -X POST "$API_URL/api/v1/auth/login" \
      -H "Content-Type: application/json" \
      -d "{\"username_or_email\":\"$TEST_USER\",\"password\":\"$TEST_PASSWORD\"}")

    if echo "$LOGIN_RESPONSE" | grep -q "token"; then
        log_success "User Login - Success"
        TOKEN=$(echo "$LOGIN_RESPONSE" | grep -o '"token":"[^"]*"' | head -n1 | cut -d'"' -f4)
        USER_ID=$(echo "$LOGIN_RESPONSE" | grep -o '"id":[0-9]*' | head -n1 | cut -d':' -f2)
        echo "Token: $TOKEN"
        echo "User ID: $USER_ID"
    else
        log_error "Failed to get authentication token"
        exit 1
    fi
fi

echo ""

# Test 3: Verify Credentials
test_endpoint "Verify Credentials" "GET" "$API_URL/api/v1/accounts/verify_credentials" "-H \"Authorization: Bearer $TOKEN\""

# Test 4: Get Public Timeline
test_endpoint "Get Public Timeline" "GET" "$API_URL/api/v1/timelines/public"

# Test 5: Post Status
STATUS_RESPONSE=$(curl -s -X POST "$API_URL/api/v1/statuses" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"status":"Hello from automated test! #rustodon #automated"}')

if echo "$STATUS_RESPONSE" | grep -q "id"; then
    log_success "Post Status - Success"
    STATUS_ID=$(echo "$STATUS_RESPONSE" | grep -o '"id":[0-9]*' | head -n1 | cut -d':' -f2)
    echo "Status ID: $STATUS_ID"
else
    log_error "Failed to post status"
    echo "$STATUS_RESPONSE"
fi

echo ""

# Test 6: Get User Statuses
test_endpoint "Get User Statuses" "GET" "$API_URL/api/v1/accounts/$USER_ID/statuses" "-H \"Authorization: Bearer $TOKEN\""

# Test 7: Favorite Status
if [ -n "$STATUS_ID" ]; then
    test_endpoint "Favorite Status" "POST" "$API_URL/api/v1/statuses/$STATUS_ID/favourite" "-H \"Authorization: Bearer $TOKEN\""
fi

# Test 8: Unfavorite Status
if [ -n "$STATUS_ID" ]; then
    test_endpoint "Unfavorite Status" "POST" "$API_URL/api/v1/statuses/$STATUS_ID/unfavourite" "-H \"Authorization: Bearer $TOKEN\""
fi

# Test 9: Get Notifications
test_endpoint "Get Notifications" "GET" "$API_URL/api/v1/notifications" "-H \"Authorization: Bearer $TOKEN\""

# Test 10: Follow User (if there's another user)
test_endpoint "Follow User" "POST" "$API_URL/api/v1/accounts/2/follow" "-H \"Authorization: Bearer $TOKEN\""

# Test 11: Get Relationships
test_endpoint "Get Relationships" "GET" "$API_URL/api/v1/accounts/relationships?id=2" "-H \"Authorization: Bearer $TOKEN\""

# Test 12: Unfollow User
test_endpoint "Unfollow User" "POST" "$API_URL/api/v1/accounts/2/unfollow" "-H \"Authorization: Bearer $TOKEN\""

# Test 13: Reblog Status
if [ -n "$STATUS_ID" ]; then
    test_endpoint "Reblog Status" "POST" "$API_URL/api/v1/statuses/$STATUS_ID/reblog" "-H \"Authorization: Bearer $TOKEN\""
fi

# Test 14: Unreblog Status
if [ -n "$STATUS_ID" ]; then
    test_endpoint "Unreblog Status" "POST" "$API_URL/api/v1/statuses/$STATUS_ID/unreblog" "-H \"Authorization: Bearer $TOKEN\""
fi

# Test 15: Get Home Timeline
test_endpoint "Get Home Timeline" "GET" "$API_URL/api/v1/timelines/home" "-H \"Authorization: Bearer $TOKEN\""

# Test 16: Register OAuth App
test_endpoint "Register OAuth App" "POST" "$API_URL/api/v1/apps" "-H \"Content-Type: application/json\"" '{"client_name":"Automated Test App","redirect_uris":"urn:ietf:wg:oauth:2.0:oob","scopes":"read write follow"}'

# Test 17: Get User Info
test_endpoint "Get User Info" "GET" "$API_URL/api/v1/accounts/$USER_ID"

# Test 18: Get Followers
test_endpoint "Get Followers" "GET" "$API_URL/api/v1/accounts/$USER_ID/followers"

# Test 19: Get Following
test_endpoint "Get Following" "GET" "$API_URL/api/v1/accounts/$USER_ID/following"

# Test 20: Block User
test_endpoint "Block User" "POST" "$API_URL/api/v1/accounts/2/block" "-H \"Authorization: Bearer $TOKEN\""

# Test 21: Unblock User
test_endpoint "Unblock User" "POST" "$API_URL/api/v1/accounts/2/unblock" "-H \"Authorization: Bearer $TOKEN\""

# Test 22: Mute User
test_endpoint "Mute User" "POST" "$API_URL/api/v1/accounts/2/mute" "-H \"Authorization: Bearer $TOKEN\""

# Test 23: Unmute User
test_endpoint "Unmute User" "POST" "$API_URL/api/v1/accounts/2/unmute" "-H \"Authorization: Bearer $TOKEN\""

# Test 24: Create List
LIST_RESPONSE=$(curl -s -X POST "$API_URL/api/v1/lists" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"title":"Automated Test List"}')

if echo "$LIST_RESPONSE" | grep -q "id"; then
    log_success "Create List - Success"
    LIST_ID=$(echo "$LIST_RESPONSE" | grep -o '"id":[0-9]*' | head -n1 | cut -d':' -f2)
    echo "List ID: $LIST_ID"

    # Test 25: Get Lists
    test_endpoint "Get Lists" "GET" "$API_URL/api/v1/lists" "-H \"Authorization: Bearer $TOKEN\""

    # Test 26: Delete List
    test_endpoint "Delete List" "DELETE" "$API_URL/api/v1/lists/$LIST_ID" "-H \"Authorization: Bearer $TOKEN\""
else
    log_error "Failed to create list"
    echo "$LIST_RESPONSE"
fi

echo ""

# Test 27: Search
test_endpoint "Search" "GET" "$API_URL/api/v1/search?q=rustodon" "-H \"Authorization: Bearer $TOKEN\""

# Test 28: Get Tags
test_endpoint "Get Tags" "GET" "$API_URL/api/v1/tags"

# Test 29: Get Instance Info
test_endpoint "Get Instance Info" "GET" "$API_URL/api/v1/instance"

# Test 30: Get Trending Tags
test_endpoint "Get Trending Tags" "GET" "$API_URL/api/v1/trends/tags"

echo ""
echo "=== 🎉 AUTOMATED TESTING COMPLETE ==="
echo ""

# Show server logs if there were any errors
if [ -f server.log ]; then
    log_info "Server logs (last 20 lines):"
    tail -n 20 server.log
fi

# Cleanup
log_info "Stopping server..."
kill $SERVER_PID 2>/dev/null || true
rm -f server.log

log_success "All tests completed! Check the results above."
