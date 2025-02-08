#!/bin/bash
set -e

# Ensure GitHub CLI is authenticated
echo "🔐 Checking GitHub CLI authentication..."
if ! gh auth status 2>/dev/null; then
  echo "🔑 Logging in to GitHub CLI using token..."
  echo "$GITHUB_TOKEN" | gh auth login --with-token
  if [ $? -ne 0 ]; then
    echo "❌ GitHub authentication failed!"
    exit 1
  fi
fi

# Fetch a new runner registration token
echo "🔄 Fetching new GitHub Actions runner token..."
if [ -z "$GITHUB_REPO" ]; then
  # Organization-wide runner
  GH_RUNNER_TOKEN=$(GH_TOKEN="$GITHUB_TOKEN" gh api --method POST -H "Accept: application/vnd.github.v3+json" \
    /orgs/$GITHUB_OWNER/actions/runners/registration-token --jq .token)
else
  # Repository-level runner
  GH_RUNNER_TOKEN=$(GH_TOKEN="$GITHUB_TOKEN" gh api --method POST -H "Accept: application/vnd.github.v3+json" \
    /repos/$GITHUB_OWNER/$GITHUB_REPO/actions/runners/registration-token --jq .token)
fi

if [ -z "$GH_RUNNER_TOKEN" ]; then
  echo "❌ Failed to fetch the runner token!"
  exit 1
fi
echo "✅ New token obtained successfully!"

# Stop and unregister the old runner
echo "🛑 Stopping and unregistering the old runner..."
./config.sh remove --token "$GH_RUNNER_TOKEN"

# Re-register the runner
echo "🔄 Registering the runner with the new token..."
if [ -z "$GITHUB_REPO" ]; then
  ./config.sh --url "https://github.com/$GITHUB_OWNER" --token "$GH_RUNNER_TOKEN" --unattended --name "$(hostname)" --replace
else
  ./config.sh --url "https://github.com/$GITHUB_OWNER/$GITHUB_REPO" --token "$GH_RUNNER_TOKEN" --unattended --name "$(hostname)" --replace
fi

# Run the GitHub Actions Runner
cleanup() {
  echo "🛑 Stopping and unregistering the runner..."
  ./config.sh remove --token "$GH_RUNNER_TOKEN"
}
trap cleanup EXIT SIGTERM SIGINT

echo "🔄 Starting runner manually..."
./run.sh &

# Wait for the process to finish (needed for signal handling)
echo "🎉 GitHub self-hosted runner has been updated and restarted!"
wait $!
