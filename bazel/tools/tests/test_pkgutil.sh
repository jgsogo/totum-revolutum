#!/usr/bin/env bash
set -euo pipefail

COMMAND="$(command -v pkgutil || true)"

if [[ -n "$COMMAND" && -x "$COMMAND" ]]; then
    echo "pkgutil found: $COMMAND"
else
    echo "Error: pkgutil command not found." >&2
    echo "You should follow the next steps:"
    echo " * Install Xcode Command Line Tools" >&2
    echo " * Add the path where it is located to '--action_env=PATH=...' to your user.bazelrc" >&2
    echo "More info at 'notes/2025-12-03-1913.md'"
    exit 1
fi
