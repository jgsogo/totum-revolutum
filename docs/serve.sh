#!/bin/bash
set -euo pipefail

# Get the directory where this script is located (docs/)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# Go up one level to the project root
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "=== Building Bazel Documentation ==="
cd "$PROJECT_ROOT"

# Build the Bazel documentation
echo "Generating Bazel rule documentation..."
bazel build //bazel:all_docs

# Copy generated docs to docs/bazel/
echo "Copying generated documentation to docs/bazel/..."
mkdir -p "$SCRIPT_DIR/bazel/rust/docs"
mkdir -p "$SCRIPT_DIR/bazel/rust/diesel"
mkdir -p "$SCRIPT_DIR/bazel/containers/postgres"

cp bazel-bin/bazel/dedent.md "$SCRIPT_DIR/bazel/"
cp bazel-bin/bazel/run_copy_to_workspace.md "$SCRIPT_DIR/bazel/"
cp bazel-bin/bazel/sh_with_runfiles_binary.md "$SCRIPT_DIR/bazel/"
cp bazel-bin/bazel/rust/defs.md "$SCRIPT_DIR/bazel/rust/"
cp bazel-bin/bazel/rust/docs/defs.md "$SCRIPT_DIR/bazel/rust/docs/"
cp bazel-bin/bazel/rust/diesel/diesel_print_schema.md "$SCRIPT_DIR/bazel/rust/diesel/"
cp bazel-bin/bazel/rust/diesel/diesel_setup.md "$SCRIPT_DIR/bazel/rust/diesel/"
cp bazel-bin/bazel/containers/postgres/with_postgres_run.md "$SCRIPT_DIR/bazel/containers/postgres/"

echo ""
echo "=== Setting up MkDocs Environment ==="

# Create a virtual environment if it doesn't exist
if [ ! -d "$SCRIPT_DIR/.venv" ]; then
    echo "Creating virtual environment..."
    python3 -m venv "$SCRIPT_DIR/.venv"
fi

# Activate virtual environment
source "$SCRIPT_DIR/.venv/bin/activate"

# Install dependencies
echo "Installing dependencies..."
pip install -q --upgrade pip
pip install -q -r "$SCRIPT_DIR/requirements.txt"

# Serve the documentation
echo ""
echo "=== Starting MkDocs Server ==="
echo "Documentation will be available at http://127.0.0.1:8000"
echo "Press Ctrl+C to stop the server"
echo ""
cd "$PROJECT_ROOT"
mkdocs serve -f docs/mkdocs.yml

# Made with Bob
