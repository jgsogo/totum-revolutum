#!/usr/bin/env bash

# Docs about Bazel workspace status and "stamping": https://bazel.build/docs/user-manual#workspace-status

# Git revision
echo "STABLE_BUILD_SCM_REVISION $(git rev-parse HEAD)"
tree_status="Clean"
git diff-index --quiet HEAD -- || {
    tree_status="Modified"
}
echo "STABLE_BUILD_SCM_STATUS ${tree_status}"
