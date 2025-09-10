#!/usr/bin/env bash

# Docs about Bazel workspace status and "stamping": https://bazel.build/docs/user-manual#workspace-status

# Git revision
echo "STABLE_BUILD_SCM_REVISION $(git rev-parse HEAD)"
tree_status="Clean"
git diff-index --quiet HEAD -- || {
    tree_status="Modified"
}
echo "STABLE_BUILD_SCM_STATUS ${tree_status}"

# These are all the applications we have here
# FIXME: Use ./script/gh/latest_git_tag.sh <prefix>
echo "STABLE_FINANCES_VERSION $(./scripts/gh/latest_git_tag.sh finances | cut -d 'v' -f 2)"
echo "STABLE_PHOTODB_VERSION $(./scripts/gh/latest_git_tag.sh photodb | cut -d 'v' -f 2)"
echo "STABLE_SYNCRONIA_VERSION $(./scripts/gh/latest_git_tag.sh syncronia | cut -d 'v' -f 2)"
echo "STABLE_BOARD_GAMES_VERSION $(./scripts/gh/latest_git_tag.sh board_games | cut -d 'v' -f 2)"

# Just print something at the end so the previous line is created even if empty
echo "---"
