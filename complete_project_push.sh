#!/bin/bash

# Complete Project Push Script for Rustodon
# Author: arkSong (arksong2018@gmail.com)
# Project: rustodon
# Target: https://github.com/arkCyber/Rustodon

set -e

echo "=== Complete Rustodon Project Push ==="
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

# Clean up temporary files
cleanup_temp_files() {
    print_status "Cleaning up temporary files..."

    # Remove temporary test files
    rm -f status_id token status
    rm -f images.jpg
    find . -name ".DS_Store" -type f -delete

    print_success "Temporary files cleaned up"
}

# Add all important project files
add_project_files() {
    print_status "Adding all important project files..."

    # Core project files
    git add README.md LICENSE CONTRIBUTING.md CODE_OF_CONDUCT.md SECURITY.md
    git add CHANGELOG.md AUTHORS.md

    # Configuration files
    git add .gitignore .gitattributes .editorconfig
    git add .dockerignore .buildpacks .foreman
    git add .rspec .haml-lint.yml .prettierrc.js .prettierignore
    git add .nvmrc .browserslistrc .annotaterb.yml
    git add .cursorrules

    # Docker files
    git add Dockerfile* docker-compose*.yml
    git add dev_auto.sh docker_test.sh docker_test_setup.sh simple_docker_test.sh

    # Documentation
    git add *.md
    git add docs/

    # Rust project files
    git add Cargo.toml Cargo.lock
    git add rustodon-*/Cargo.toml
    git add rustodon-*/src/

    # Database and migrations
    git add db/
    git add migrations/
    git add rustodon-migrations/

    # Configuration
    git add config.ru Procfile Procfile.dev
    git add Gemfile Gemfile.lock
    git add package.json yarn.lock
    git add tsconfig.json jsconfig.json
    git add vite.config.mts vitest.config.mts vitest.shims.d.ts
    git add eslint.config.mjs stylelint.config.js
    git add lint-staged.config.js ide-helper.js
    git add crowdin.yml scalingo.json Vagrantfile

    # Scripts and tests
    git add scripts/
    git add tests/
    git add spec/
    git add *.sh

    # Public assets
    git add public/

    # Storage and streaming
    git add storage/
    git add streaming/

    # Development files
    git add .devcontainer/
    git add priv-config/
    git add dist/

    # Vendor (if needed)
    if [ -d "vendor" ] && [ "$(ls -A vendor)" ]; then
        git add vendor/
    fi

    print_success "All project files added to staging"
}

# Check for changes
check_changes() {
    print_status "Checking for changes..."

    if git diff --cached --quiet; then
        print_warning "No staged changes to commit"
        return 1
    else
        local count=$(git diff --cached --name-only | wc -l)
        print_success "Found $count files staged for commit"
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

    if git push origin main; then
        print_success "Successfully pushed to GitHub"
        return 0
    else
        print_error "Failed to push to GitHub"
        return 1
    fi
}

# Show repository status
show_status() {
    print_status "Repository Status:"
    echo "  - Branch: $(git branch --show-current)"
    echo "  - Remote: $(git remote get-url origin)"
    echo "  - Last commit: $(git log -1 --oneline)"
    echo "  - Total files: $(git ls-files | wc -l)"
    echo "  - Repository size: $(du -sh .git 2>/dev/null | cut -f1 || echo 'unknown')"
}

# Main function
main() {
    print_status "Starting complete project push..."

    # Clean up and add files
    cleanup_temp_files
    add_project_files

    # Check if there are changes to commit
    if check_changes; then
        # Commit and push
        if commit_changes "feat: complete Rustodon project with all modules and documentation"; then
            if push_to_github; then
                print_success "Complete project push completed successfully!"
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
        print_warning "No changes to push"
        show_status
    fi
}

# Run main function
main

echo
print_success "Complete project push process finished!"
echo
print_status "Next steps:"
echo "1. Visit https://github.com/arkCyber/Rustodon to verify all files"
echo "2. Check that all Rustodon modules are present"
echo "3. Verify documentation and scripts are uploaded"
echo "4. Review the repository structure"
echo
print_status "Repository URL: https://github.com/arkCyber/Rustodon"
