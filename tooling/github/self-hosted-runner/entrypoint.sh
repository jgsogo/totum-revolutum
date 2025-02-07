#!/bin/bash
set -e

# Ensure necessary variables are set
if [[ -z "$GH_REPO_URL" || -z "$GH_RUNNER_TOKEN" ]]; then
  echo "Missing GH_REPO_URL or GH_RUNNER_TOKEN. Exiting."
  exit 1
fi

# Register the GitHub Actions Runner
./config.sh --url "$GH_REPO_URL" --token "$GH_RUNNER_TOKEN" --unattended --name "$(hostname)" --replace

# Run the GitHub Actions Runner
cleanup() {
  echo "Removing runner..."
  ./config.sh remove --token "$GH_RUNNER_TOKEN"
}
trap cleanup EXIT

exec ./run.sh
