#!/bin/bash

# Rustodon Performance Test Script
echo "🚀 Rustodon Performance Test Suite"
echo "=================================="

SERVER_URL="http://localhost:3000"
CONCURRENT_USERS=1000
TEST_DURATION=60

echo "Server URL: $SERVER_URL"
echo "Concurrent Users: $CONCURRENT_USERS"
echo "Test Duration: ${TEST_DURATION}s"
echo ""

# Check if server is running
echo "🔍 Checking if server is running..."
if curl -s "$SERVER_URL/api/v1/instance" > /dev/null 2>&1; then
    echo "✅ Server is running"
else
    echo "❌ Server is not running on $SERVER_URL"
    echo "Please start the server first: cargo run -p rustodon-server --release"
    exit 1
fi

# Install hey if not available
if ! command -v hey &> /dev/null; then
    echo "🔧 Installing hey..."
    if [[ "$OSTYPE" == "darwin"* ]]; then
        brew install hey
    else
        echo "Please install hey manually: https://github.com/rakyll/hey"
        exit 1
    fi
fi

# Create results directory
RESULTS_DIR="performance_results_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$RESULTS_DIR"

echo "📊 Running performance tests..."
echo ""

# Test endpoints
ENDPOINTS=(
    "/api/v1/instance"
    "/api/v1/apps"
    "/api/v1/accounts/verify_credentials"
    "/api/v1/statuses"
    "/api/v1/timelines/home"
    "/api/v1/timelines/public"
)

for endpoint in "${ENDPOINTS[@]}"; do
    echo "🔥 Testing $endpoint with $CONCURRENT_USERS concurrent users..."
    
    hey -n $CONCURRENT_USERS -c $CONCURRENT_USERS -z ${TEST_DURATION}s \
        -H "Content-Type: application/json" \
        "$SERVER_URL$endpoint" > "$RESULTS_DIR/hey_$(echo $endpoint | tr / _).log" 2>&1
    
    echo "✅ Completed test for $endpoint"
done

echo ""
echo "🎉 Performance testing completed!"
echo "📁 Results saved in: $RESULTS_DIR"
echo ""
echo "📋 Summary:"
for log_file in "$RESULTS_DIR"/hey_*.log; do
    if [ -f "$log_file" ]; then
        endpoint=$(basename "$log_file" .log | sed "s/hey_//" | tr "_" "/")
        echo "  $endpoint:"
        tail -n 10 "$log_file" | grep -E "(Requests/sec|Average|Total)"
    fi
done

