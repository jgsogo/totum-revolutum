#!/usr/bin/env bash
set -euo pipefail

# A script to check that the current working copy is at the HEAD
# of the default branch of the GitHub repository.

# Ensure we are inside a git repository
if ! git rev-parse --git-dir >/dev/null 2>&1; then
  echo "❌ Not inside a git repository"
  exit 1
fi

# Fetch remote refs (quietly)
git fetch origin >/dev/null

# Figure out the default branch (remote HEAD points to it)
default_branch_ref=$(git symbolic-ref refs/remotes/origin/HEAD 2>/dev/null || true)

if [[ -z "$default_branch_ref" ]]; then
  echo "❌ Could not determine default branch (is 'origin' set?)"
  exit 1
fi

default_branch=${default_branch_ref#refs/remotes/origin/}
echo "ℹ️ Default branch is: $default_branch"

# Compare local HEAD to remote default branch
remote_sha=$(git rev-parse "origin/$default_branch")
local_sha=$(git rev-parse HEAD)

if [[ "$local_sha" == "$remote_sha" ]]; then
  echo "✅ Working copy is at HEAD of $default_branch"
  exit 0
else
  echo "⚠️ Working copy is NOT at HEAD of $default_branch"
  echo "  Local:  $local_sha"
  echo "  Remote: $remote_sha"
  exit 1
fi
