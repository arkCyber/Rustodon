#!/bin/bash
# Quick Rustodon API Test Script

echo "🚀 Quick Rustodon API Test"
echo "=========================="

API_URL="http://localhost:3000"

# Check if server is running
echo "1. Checking if server is running..."
if curl -s "$API_URL/api/v1/health" > /dev/null 2>&1; then
    echo "✅ Server is running!"
else
    echo "❌ Server is not running. Please start it first:"
    echo "   cd /Users/arksong/mastodon@rustodon_副本/rustodon"
    echo "   RUST_LOG=info cargo run -p rustodon-server &"
    exit 1
fi

echo ""
echo "2. Testing Health Check:"
curl -s "$API_URL/api/v1/health" | head -c 100
echo "..."

echo ""
echo "3. Testing Public Timeline:"
curl -s "$API_URL/api/v1/timelines/public" | head -c 200
echo "..."

echo ""
echo "4. Testing Instance Info:"
curl -s "$API_URL/api/v1/instance" | head -c 200
echo "..."

echo ""
echo "5. Testing User Registration:"
REG_RESPONSE=$(curl -s -X POST "$API_URL/api/v1/auth/register" \
  -H "Content-Type: application/json" \
  -d '{"username":"quicktest","email":"quicktest@example.com","password":"testpass","agreement":true,"locale":"en"}')

if echo "$REG_RESPONSE" | grep -q "token"; then
    echo "✅ Registration successful!"
    TOKEN=$(echo "$REG_RESPONSE" | grep -o '"token":"[^"]*"' | head -n1 | cut -d'"' -f4)
    USER_ID=$(echo "$REG_RESPONSE" | grep -o '"id":[0-9]*' | head -n1 | cut -d':' -f2)
    echo "   Token: $TOKEN"
    echo "   User ID: $USER_ID"

    echo ""
    echo "6. Testing Post Status:"
    STATUS_RESPONSE=$(curl -s -X POST "$API_URL/api/v1/statuses" \
      -H "Authorization: Bearer $TOKEN" \
      -H "Content-Type: application/json" \
      -d '{"status":"Hello from quick test! #rustodon"}')

    if echo "$STATUS_RESPONSE" | grep -q "id"; then
        echo "✅ Status posted successfully!"
        STATUS_ID=$(echo "$STATUS_RESPONSE" | grep -o '"id":[0-9]*' | head -n1 | cut -d':' -f2)
        echo "   Status ID: $STATUS_ID"

        echo ""
        echo "7. Testing Favorite Status:"
        curl -s -X POST "$API_URL/api/v1/statuses/$STATUS_ID/favourite" \
          -H "Authorization: Bearer $TOKEN" | head -c 100
        echo "..."
    else
        echo "❌ Failed to post status"
        echo "$STATUS_RESPONSE"
    fi
else
    echo "❌ Registration failed or user already exists"
    echo "$REG_RESPONSE"
fi

echo ""
echo "🎉 Quick test completed!"
