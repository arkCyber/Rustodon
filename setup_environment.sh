#!/bin/bash

# Setup environment for Rustodon development
echo "Setting up Rustodon development environment..."

# Create .env file if it doesn't exist
if [ ! -f .env ]; then
    cat > .env << EOF
# Database configuration
DATABASE_URL=postgresql://rustodon:rustodon@localhost:5432/rustodon

# Redis configuration
REDIS_URL=redis://localhost:6379

# Server configuration
RUST_LOG=debug
RUST_BACKTRACE=1

# Development settings
ENVIRONMENT=development
EOF
    echo "Created .env file"
fi

# Export environment variables for current session
export DATABASE_URL=postgresql://rustodon:rustodon@localhost:5432/rustodon
export REDIS_URL=redis://localhost:6379
export RUST_LOG=debug
export RUST_BACKTRACE=1
export ENVIRONMENT=development

echo "Environment variables set:"
echo "DATABASE_URL=$DATABASE_URL"
echo "REDIS_URL=$REDIS_URL"
echo "RUST_LOG=$RUST_LOG"
echo "ENVIRONMENT=$ENVIRONMENT"

echo "Environment setup complete!"
