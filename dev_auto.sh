#!/bin/bash
set -e

# 1. Ensure rustodon-server is in workspace members
cd "$(dirname "$0")"
CARGO_TOML="Cargo.toml"
if ! grep -q '"rustodon-server"' "$CARGO_TOML"; then
  echo "Adding rustodon-server to workspace members..."
  sed -i '' '/members = \[/a\
    "rustodon-server",' "$CARGO_TOML"
fi

# 2. Ensure tracing = "0.1" in [workspace.dependencies]
if grep -A 2 '\[workspace.dependencies\]' "$CARGO_TOML" | grep -q 'tracing'; then
  sed -i '' '/\[workspace.dependencies\]/,/^$/ s/^tracing.*$/tracing = "0.1"/' "$CARGO_TOML"
else
  sed -i '' '/\[workspace.dependencies\]/a\
tracing = "0.1"' "$CARGO_TOML"
fi

# 3. Format, check, lint, test
cargo fmt --all
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

# 4. Kill process on port 3000 if exists
PORT=3000
PID=$(lsof -ti tcp:$PORT)
if [ -n "$PID" ]; then
  echo "Killing process on port $PORT (PID $PID)..."
  kill -9 $PID
fi

# 5. Run rustodon-server
cargo run -p rustodon-server
