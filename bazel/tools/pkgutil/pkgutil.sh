#!/bin/bash
set -euo pipefail

PKGUTIL="/usr/sbin/pkgutil"

if [[ -x "$PKGUTIL" ]]; then
    "$PKGUTIL" "$@"
else
    echo "Error: pkgutil not found at $PKGUTIL" >&2
    echo "This script requires macOS pkgutil (part of Xcode Command Line Tools or system utilities)." >&2
    exit 1
fi
