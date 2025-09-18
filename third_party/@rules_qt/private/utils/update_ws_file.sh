#!/bin/bash

set -e  # Exit on error

SOURCE="$1"
TARGET="$2"

# Check that the source file exists
if [[ ! -f "$SOURCE" ]]; then
    echo "Source file '$SOURCE' does not exist."
    exit 1
fi

# Check that the target file exists
if [[ ! -f "$TARGET" ]]; then
    echo "Target file '$TARGET' does not exist."
    exit 1
fi

# Overwrite the contents without touching metadata
cat "$SOURCE" > "$TARGET"
