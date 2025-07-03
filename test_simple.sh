#!/bin/bash
set -e

echo "Running simplified Rustodon test..."

cd rustodon

# Kill any process on port 3000
PORT=3000
PID=$(lsof -ti tcp:$PORT 2>/dev/null || true)
if [ -n "$PID" ]; then
    echo "Killing process on port $PORT (PID $PID)..."
    kill -9 $PID
fi

# Try to run just the server without full workspace build
echo "Attempting to run rustodon-server directly..."
cd rustodon-server
cargo run --bin rustodon-server 2>&1 | head -20

echo "Test completed."
