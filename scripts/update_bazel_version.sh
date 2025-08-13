#!/usr/bin/env bash
set -euo pipefail

BAZELVERSION_FILE="${1:-.bazelversion}"

if [[ ! -f "$BAZELVERSION_FILE" ]]; then
    echo "❌ .bazelversion file not found at $BAZELVERSION_FILE"
    exit 1
fi

echo "📥 Fetching latest Bazel release version..."
LATEST=$(curl -s https://api.github.com/repos/bazelbuild/bazel/releases/latest | jq -r '.tag_name' | sed 's/^release-//')

if [[ -z "$LATEST" ]]; then
    echo "❌ Could not fetch latest Bazel version"
    exit 1
fi

echo "🔄 Updating .bazelversion to use Bazel $LATEST"

# Replace file content
echo "$LATEST" > "$BAZELVERSION_FILE"

echo "✅ Updated $BAZELVERSION_FILE to Bazel $LATEST (backup saved as ${BAZELVERSION_FILE}.bak)"
