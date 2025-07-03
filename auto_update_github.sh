#!/bin/bash

# Auto Update GitHub Repository Script for Rustodon
# Author: arkSong (arksong2018@gmail.com)
# Project: rustodon
# Target: https://github.com/arkCyber/Rustodon

set -e

echo "=== Rustodon Auto GitHub Update ==="
echo "Starting at: $(date)"
echo "Target Repository: https://github.com/arkCyber/Rustodon"
echo

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

# Check if we're in a git repository
check_git_repo() {
    if [ ! -d ".git" ]; then
        print_error "Not in a git repository. Please run this script from the project root."
        exit 1
    fi
    print_success "Git repository detected"
}

# Check remote repository
check_remote() {
    print_status "Checking remote repository..."

    if ! git remote get-url origin >/dev/null 2>&1; then
        print_status "Adding remote origin..."
        git remote add origin https://github.com/arkCyber/Rustodon.git
    fi

    REMOTE_URL=$(git remote get-url origin)
    print_success "Remote origin: $REMOTE_URL"
}

# Clean up temporary files
cleanup_files() {
    print_status "Cleaning up temporary files..."

    # Remove temporary test files
    rm -f /tmp/response.json /tmp/test.png
    rm -f status_id token status
    find . -name ".DS_Store" -type f -delete
    rm -f images.jpg

    print_success "Temporary files cleaned up"
}

# Stage important files
stage_files() {
    print_status "Staging important files..."

    # Add core documentation and scripts
    git add PROJECT_SUMMARY.md API_TEST_REPORT.md
    git add comprehensive_curl_test.sh advanced_api_test.sh
    git add simple_test_server.py setup_github.sh auto_update_github.sh

    # Add Rust project files
    git add Cargo.toml Cargo.lock
    git add rustodon-*/Cargo.toml

    # Add configuration files
    git add .gitignore .env.example
    git add docker-compose*.yml Dockerfile*

    # Add documentation
    if [ -d "docs" ]; then
        git add docs/
    fi

    # Add scripts
    if [ -d "scripts" ]; then
        git add scripts/
    fi

    print_success "Files staged for commit"
}

# Check for changes
check_changes() {
    print_status "Checking for changes..."

    if git diff --cached --quiet; then
        print_warning "No staged changes to commit"
        return 1
    else
        print_success "Changes detected and staged"
        return 0
    fi
}

# Commit changes
commit_changes() {
    local commit_message="$1"

    print_status "Committing changes..."
    print_status "Commit message: $commit_message"

    if git commit -m "$commit_message"; then
        print_success "Changes committed successfully"
        return 0
    else
        print_error "Failed to commit changes"
        return 1
    fi
}

# Push to GitHub
push_to_github() {
    print_status "Pushing to GitHub..."

    # Try to push to main branch
    if git push -u origin main; then
        print_success "Successfully pushed to GitHub"
        return 0
    else
        print_warning "Push to main failed, trying to force push..."
        if git push -u origin main --force; then
            print_success "Successfully force pushed to GitHub"
            return 0
        else
            print_error "Failed to push to GitHub"
            return 1
        fi
    fi
}

# Show repository status
show_status() {
    print_status "Repository Status:"
    echo "  - Branch: $(git branch --show-current)"
    echo "  - Remote: $(git remote get-url origin)"
    echo "  - Last commit: $(git log -1 --oneline)"
    echo "  - Status: $(git status --porcelain | wc -l) files changed"
}

# Main update function
main_update() {
    local commit_message="${1:-"feat: update Rustodon project with API tests and documentation"}"

    print_status "Starting auto update process..."

    # Check prerequisites
    check_git_repo
    check_remote

    # Clean up and stage files
    cleanup_files
    stage_files

    # Check if there are changes to commit
    if check_changes; then
        # Commit and push
        if commit_changes "$commit_message"; then
            if push_to_github; then
                print_success "Auto update completed successfully!"
                echo
                print_status "Repository updated at: https://github.com/arkCyber/Rustodon"
                show_status
            else
                print_error "Failed to push to GitHub"
                exit 1
            fi
        else
            print_error "Failed to commit changes"
            exit 1
        fi
    else
        print_warning "No changes to update"
        show_status
    fi
}

# Quick update function (minimal changes)
quick_update() {
    print_status "Performing quick update..."

    # Only stage essential files
    git add PROJECT_SUMMARY.md API_TEST_REPORT.md
    git add comprehensive_curl_test.sh advanced_api_test.sh
    git add simple_test_server.py

    if check_changes; then
        commit_changes "docs: update API test reports and documentation"
        push_to_github
    else
        print_warning "No essential changes to update"
    fi
}

# Force update function
force_update() {
    print_status "Performing force update..."

    # Stage all files
    git add .

    if check_changes; then
        commit_changes "feat: complete project update with all files"
        push_to_github
    else
        print_warning "No changes to force update"
    fi
}

# Show help
show_help() {
    echo "Usage: $0 [OPTION] [COMMIT_MESSAGE]"
    echo
    echo "Options:"
    echo "  -h, --help     Show this help message"
    echo "  -q, --quick    Quick update (essential files only)"
    echo "  -f, --force    Force update (all files)"
    echo "  -s, --status   Show repository status"
    echo
    echo "Examples:"
    echo "  $0                                    # Standard update"
    echo "  $0 \"feat: add new API endpoints\"    # Custom commit message"
    echo "  $0 -q                                 # Quick update"
    echo "  $0 -f                                 # Force update"
    echo "  $0 -s                                 # Show status"
}

# Parse command line arguments
case "${1:-}" in
    -h|--help)
        show_help
        exit 0
        ;;
    -q|--quick)
        main_update
        quick_update
        ;;
    -f|--force)
        main_update
        force_update
        ;;
    -s|--status)
        show_status
        exit 0
        ;;
    "")
        main_update
        ;;
    *)
        main_update "$1"
        ;;
esac

echo
print_success "Auto update process completed!"
echo
print_status "Next steps:"
echo "1. Visit https://github.com/arkCyber/Rustodon to verify updates"
echo "2. Check if all files are properly uploaded"
echo "3. Review the commit history"
echo "4. Update README.md if needed"
echo
print_status "Repository URL: https://github.com/arkCyber/Rustodon"
