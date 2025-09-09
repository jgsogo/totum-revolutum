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
echo "STABLE_FINANCES_VERSION $(git tag --list 'finances*' | sort -V | tail -1 | cut -d 'v' -f 2)"
echo "STABLE_PHOTODB_VERSION $(git tag --list 'photodb*' | sort -V | tail -1 | cut -d 'v' -f 2)"
echo "STABLE_SYNCRONIA_VERSION $(git tag --list 'syncronia*' | sort -V | tail -1 | cut -d 'v' -f 2)"
echo "STABLE_BOARD_GAMES_VERSION $(git tag --list 'board_games*' | sort -V | tail -1 | cut -d 'v' -f 2)"

# Just print something at the end so the previous line is created even if empty
echo "---"
