#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 ]]; then
    echo "Usage: $0 <prefix>"
    exit 1
fi

prefix="$1"

# Get latest tag for the given prefix
latest_tag=$(git tag --list "${prefix}-v*" | sort -V | tail -1)

echo "$latest_tag"
