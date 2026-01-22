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

# Serve the documentation
echo "Starting MkDocs server..."
echo "Documentation will be available at http://127.0.0.1:8000"
echo "Press Ctrl+C to stop the server"
cd "$DOCS_DIR"
mkdocs serve

# Made with Bob
