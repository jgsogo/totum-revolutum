#!/bin/bash
set -euo pipefail

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DOCS_DIR="$SCRIPT_DIR"

echo "Setting up MkDocs environment..."

# Create a virtual environment if it doesn't exist
if [ ! -d "$DOCS_DIR/.venv" ]; then
    echo "Creating virtual environment..."
    python3 -m venv "$DOCS_DIR/.venv"
fi

# Activate virtual environment
source "$DOCS_DIR/.venv/bin/activate"

# Install dependencies
echo "Installing dependencies..."
pip install -q --upgrade pip
pip install -q -r "$DOCS_DIR/requirements.txt"

# Build the documentation
echo "Building documentation site..."
cd "$DOCS_DIR"
mkdocs build

echo ""
echo "Documentation built successfully!"
echo "Output directory: $DOCS_DIR/site"
echo ""
echo "To view the site:"
echo "  1. Open $DOCS_DIR/site/index.html in your browser"
echo "  2. Or run: python3 -m http.server 8000 --directory $DOCS_DIR/site"

# Made with Bob
