#!/usr/bin/env bash

set -euo pipefail

JSON_FILE="$1"
STRIP_PREFIX="$2"
OUTPUT_TAR="$3"

TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

echo "📦 Reading file list from $JSON_FILE..."
jq -r '.[]' "$JSON_FILE" | while read -r file; do
    echo "file: $file"
  relative_path="${file#$STRIP_PREFIX/}"
    echo "relative_path: $relative_path"
  dest="$TMP_DIR/$relative_path"
  mkdir -p "$(dirname "$dest")"
  cp "$file" "$dest"
done

echo "🗜️ Creating archive $OUTPUT_TAR..."
tar -czf "$OUTPUT_TAR" -C "$TMP_DIR" .

echo "✅ Archive $OUTPUT_TAR created with relative paths."
