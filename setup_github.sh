#!/bin/bash

# Setup GitHub Repository for Rustodon
# Author: arkSong (arksong2018@gmail.com)

echo "=== Rustodon GitHub Setup ==="
echo "Starting at: $(date)"
echo

# Clean up temporary files
echo "[INFO] Cleaning up temporary files..."
rm -f /tmp/response.json /tmp/test.png
rm -f status_id token status
find . -name ".DS_Store" -type f -delete
rm -f images.jpg
echo "[SUCCESS] Temporary files cleaned up"
echo

# Create .gitignore if it doesn't exist
if [ ! -f ".gitignore" ]; then
    echo "[INFO] Creating .gitignore file..."
    cat > .gitignore << 'EOF'
# Rust
/target/
**/*.rs.bk
Cargo.lock

# IDE
.vscode/
.idea/
*.swp
*.swo
*~

# OS
.DS_Store
.DS_Store?
._*
.Spotlight-V100
.Trashes
ehthumbs.db
Thumbs.db

# Environment variables
.env
.env.local
.env.production

# Temporary files
/tmp/
*.tmp
*.temp
status_id
token
status
images.jpg

# Database
*.db
*.sqlite
*.sqlite3

# Node.js
node_modules/
npm-debug.log*
yarn-debug.log*
yarn-error.log*

# Python
__pycache__/
*.py[cod]
*$py.class
*.so
.Python
env/
venv/
.venv/

# Build outputs
dist/
build/
*.o
*.a

# Backup files
*.bak
*.backup
*~
EOF
    echo "[SUCCESS] .gitignore file created"
else
    echo "[SUCCESS] .gitignore file already exists"
fi
echo

# Initialize Git repository
if [ ! -d ".git" ]; then
    echo "[INFO] Initializing Git repository..."
    git init
    echo "[SUCCESS] Git repository initialized"
else
    echo "[SUCCESS] Git repository already exists"
fi

# Add all files
git add .

# Check if there are changes to commit
if git diff --cached --quiet; then
    echo "[WARNING] No changes to commit"
else
    echo "[INFO] Changes staged for commit"
fi

echo
echo "[SUCCESS] GitHub setup completed!"
echo
echo "[INFO] Next steps:"
echo "1. Review the created files"
echo "2. Commit your changes:"
echo "   git commit -m 'Initial commit: Rustodon project setup'"
echo "3. Add remote repository:"
echo "   git remote add origin https://github.com/yourusername/rustodon.git"
echo "4. Push to GitHub:"
echo "   git push -u origin main"
echo
echo "[INFO] Created files:"
echo "  - .gitignore (Git ignore rules)"
echo "  - API_TEST_REPORT.md (comprehensive test report)"
echo "  - comprehensive_curl_test.sh (basic API tests)"
echo "  - advanced_api_test.sh (40 endpoint tests)"
echo "  - simple_test_server.py (Python test server)"
echo
