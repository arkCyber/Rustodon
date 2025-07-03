#!/bin/bash
# Local Build Script for Rustodon
# Author: arkSong (arksong2018@gmail.com)
# Project: rustodon
#
# This script builds Rustodon using the simple Dockerfile to avoid network issues
# Usage: ./scripts/build-local.sh [build|run|clean]

set -e

# Colors for output
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

# Check if Docker is running
check_docker() {
    if ! docker info > /dev/null 2>&1; then
        log_error "Docker is not running. Please start Docker Desktop or Docker daemon."
        exit 1
    fi
    log_success "Docker is running"
}

# Build the Docker image using simple Dockerfile
build_image() {
    log_info "Building Rustodon using simple Dockerfile..."
    docker build -f Dockerfile.simple -t rustodon:local .
    log_success "Docker image built successfully"
}

# Run the container
run_container() {
    log_info "Running Rustodon container..."
    docker run --rm \
        --name rustodon-local \
        -e DATABASE_URL=postgres://rustodon:rustodon@host.docker.internal:5432/rustodon \
        -e RUST_LOG=info \
        -p 3000:3000 \
        rustodon:local
}

# Clean up
clean_up() {
    log_info "Cleaning up Docker resources..."
    docker rmi rustodon:local 2>/dev/null || true
    docker system prune -f
    log_success "Cleanup completed"
}

# Show help
show_help() {
    echo "Usage: $0 [build|run|clean|help]"
    echo ""
    echo "Commands:"
    echo "  build   - Build the Docker image using simple Dockerfile"
    echo "  run     - Run the container (requires database)"
    echo "  clean   - Clean up Docker resources"
    echo "  help    - Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 build                    # Build the image"
    echo "  $0 run                      # Run the container"
    echo "  docker-compose -f docker-compose.simple.yml up db  # Start database first"
}

# Main function
main() {
    check_docker

    case "${1:-help}" in
        build)
            build_image
            ;;
        run)
            run_container
            ;;
        clean)
            clean_up
            ;;
        help|*)
            show_help
            ;;
    esac
}

# Run main function
main "$@"
