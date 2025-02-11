#!/bin/bash
set -e

####
# DOCKER
####

function wait_for_process () {
    local max_time_wait=30
    local process_name="$1"
    local waited_sec=0
    while ! pgrep "$process_name" >/dev/null && ((waited_sec < max_time_wait)); do
        echo "Process $process_name is not running yet. Retrying in 1 seconds"
        echo "Waited $waited_sec seconds of $max_time_wait seconds"
        sleep 1
        ((waited_sec=waited_sec+1))
        if ((waited_sec >= max_time_wait)); then
            return 1
        fi
    done
    return 0
}

echo "🔄 Waiting for docker to be running"
sudo /usr/bin/dockerd &
wait_for_process dockerd
if [ $? -ne 0 ]; then
    echo "❌ dockerd is not running after max time"
    exit 1
else
    echo "✅ dockerd is running"
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
# EXECUTE THE RUNNER
####

DOCKER_GROUP_ID=$(stat -c %g /var/run/docker.sock)
DOCKER_GROUP_NAME=$(getent group "$DOCKER_GROUP_ID" | cut -d: -f1)
exec sg "$DOCKER_GROUP_NAME" /runner.sh


# TODO:
# - Use supervisor to ensure that dockerd (line 25) and the GH runner (line 103) are actually running
# - Clean docker from time to time.
#   - It might happen that we are not killing all the running images
#   - Clean docker cache: docker system prune.
