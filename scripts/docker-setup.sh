#!/bin/bash
# Docker Setup Script for Rustodon
# Author: arkSong (arksong2018@gmail.com)
# Project: rustodon
#
# This script automates the Docker environment setup for Rustodon
# Usage: ./scripts/docker-setup.sh [build|start|stop|restart|clean]

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

# Build the Docker image
build_image() {
    log_info "Building Rustodon Docker image..."
    docker-compose build --no-cache
    log_success "Docker image built successfully"
}

# Start the services
start_services() {
    log_info "Starting Rustodon services..."
    docker-compose up -d
    log_success "Services started successfully"

    # Wait for services to be ready
    log_info "Waiting for services to be ready..."
    sleep 10

    # Check service status
    docker-compose ps
}

# Stop the services
stop_services() {
    log_info "Stopping Rustodon services..."
    docker-compose down
    log_success "Services stopped successfully"
}

# Restart the services
restart_services() {
    log_info "Restarting Rustodon services..."
    docker-compose down
    docker-compose up -d
    log_success "Services restarted successfully"
}

# Clean up Docker resources
clean_resources() {
    log_info "Cleaning up Docker resources..."
    docker-compose down -v --remove-orphans
    docker system prune -f
    log_success "Docker resources cleaned up"
}

# Show service logs
show_logs() {
    log_info "Showing service logs..."
    docker-compose logs -f
}

# Show service status
show_status() {
    log_info "Service status:"
    docker-compose ps
}

# Main function
main() {
    check_docker

    case "${1:-help}" in
        build)
            build_image
            ;;
        start)
            start_services
            ;;
        stop)
            stop_services
            ;;
        restart)
            restart_services
            ;;
        clean)
            clean_resources
            ;;
        logs)
            show_logs
            ;;
        status)
            show_status
            ;;
        help|*)
            echo "Usage: $0 [build|start|stop|restart|clean|logs|status]"
            echo ""
            echo "Commands:"
            echo "  build   - Build the Docker image"
            echo "  start   - Start the services"
            echo "  stop    - Stop the services"
            echo "  restart - Restart the services"
            echo "  clean   - Clean up Docker resources"
            echo "  logs    - Show service logs"
            echo "  status  - Show service status"
            echo "  help    - Show this help message"
            ;;
    esac
}

# Run main function
main "$@"
