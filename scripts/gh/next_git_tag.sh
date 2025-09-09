#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 ]]; then
    echo "Usage: $0 <prefix>"
    exit 1
fi

prefix="$1"
today=$(date +%Y.%m.%d)

# Full date prefix (e.g. prefix-v2025.09.09)
date_tag="${prefix}-v${today}"

# Get today's tags with this prefix
tags_today=$(git tag --list "${date_tag}*" | sort -V)

if [[ -z "$tags_today" ]]; then
    # No tag yet for today → just the date tag
    next_tag="${date_tag}"
else
    # Extract numeric suffixes
    last_num=$(echo "$tags_today" \
        | sed -E "s/^${date_tag}(-([0-9]+))?$/\2/" \
        | sort -n \
        | tail -n1)

    if [[ -z "$last_num" ]]; then
        # Only the plain date tag exists → start numbering at 1
        next_tag="${date_tag}-1"
    else
        # Increment from the last number
        next_tag="${date_tag}-$((last_num + 1))"
    fi
fi

echo "$next_tag"
