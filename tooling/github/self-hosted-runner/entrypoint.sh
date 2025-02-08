#!/bin/bash
set -e

####
# DOCKER
#### 

# Ensure Docker socket is accessible
if [ ! -S /var/run/docker.sock ]; then
    echo "❌ Docker socket not found! Make sure to mount it with -v /var/run/docker.sock:/var/run/docker.sock"
    exit 1
fi

# Fix permissions if needed
DOCKER_GROUP_ID=$(stat -c %g /var/run/docker.sock)
DOCKER_GROUP_NAME=$(getent group "$DOCKER_GROUP_ID" | cut -d: -f1)
# If the group doesn't exist, create it
if [ -z "$DOCKER_GROUP_NAME" ]; then
    DOCKER_GROUP_NAME="dockersocket"
    sudo groupadd -g "$DOCKER_GROUP_ID" "$DOCKER_GROUP_NAME"
fi
# Add the current user to the group
sudo usermod -aG "$DOCKER_GROUP_NAME" $RUNNER_USER
echo "✅ User added to group $DOCKER_GROUP_NAME ($DOCKER_GROUP_ID)"

####
# GH CLI
#### 

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

####
# SELF-HOSTED RUNNER
#### 

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
