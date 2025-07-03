#!/bin/bash

echo "Setting up Rustodon database..."

# Database configuration
DB_NAME="rustodon"
DB_USER="rustodon"
DB_PASSWORD="rustodon"
DB_HOST="localhost"
DB_PORT="5432"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if PostgreSQL is running
print_status "Checking PostgreSQL connection..."
if ! pg_isready -h $DB_HOST -p $DB_PORT -U $DB_USER > /dev/null 2>&1; then
    print_error "PostgreSQL is not running or not accessible"
    print_status "Please start PostgreSQL and ensure it's running on $DB_HOST:$DB_PORT"
    exit 1
fi

# Create database if it doesn't exist
print_status "Creating database if it doesn't exist..."
createdb -h $DB_HOST -p $DB_PORT -U $DB_USER $DB_NAME 2>/dev/null || print_warning "Database already exists or creation failed"

# Set up environment variables for sqlx
export DATABASE_URL="postgres://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_NAME"

# Run migrations
print_status "Running database migrations..."
cargo run -p rustodon-migrations

if [ $? -eq 0 ]; then
    print_status "Migrations completed successfully"
else
    print_error "Migrations failed"
    exit 1
fi

# Test database connection
print_status "Testing database connection..."
cargo check -p rustodon-db

if [ $? -eq 0 ]; then
    print_status "Database setup completed successfully!"
    print_status "You can now run the server with: cargo run -p rustodon-server"
else
    print_error "Database connection test failed"
    exit 1
fi
