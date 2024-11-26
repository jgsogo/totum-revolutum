#!/usr/bin/env bash
set -o pipefail -o errexit -o nounset


%BASH_RLOCATION_FUNCTION%

runfiles_export_envvars

readonly DOCKER_CLI="$(rlocation "%DOCKER_CLI%")"

if [ -f "$DOCKER_CLI" ]; then
    CONTAINER_CLI="$DOCKER_CLI"
elif command -v docker &> /dev/null; then
    CONTAINER_CLI="docker"
elif command -v podman &> /dev/null; then
    CONTAINER_CLI="podman"
else
    echo >&2 "Neither docker or podman could be found."
    echo >&2 "To use a different container runtime, pass an executable to the 'docker_cli' attribute."
    exit 1
fi

DOCKER_COMPOSE_COMMAND="$CONTAINER_CLI compose --project-name=%PROJECT_NAME% %DOCKER_COMPOSE_FILES% --env-file=$(pwd)/%ENV_FILE%"

# Ensure we execute the docker compose DOWN
function finish {
    echo "docker compose down"
    $DOCKER_COMPOSE_COMMAND down --volumes --timeout=30
}
trap finish EXIT
trap finish INT  # Capture Ctrl_C (SIGINT)

# Start docker compose UP
$DOCKER_COMPOSE_COMMAND config %SERVICES%
$DOCKER_COMPOSE_COMMAND up %SERVICES% --wait --wait-timeout 20

# Get the external port for internal 5432
SERVICE_NAME="db"
SERVICE_PORT=5432
CONTAINER_HOST_AND_PORT=$($DOCKER_COMPOSE_COMMAND port $SERVICE_NAME $SERVICE_PORT | head -n 1)
CONTAINER_HOST=$(echo "$CONTAINER_HOST_AND_PORT" | cut -d ":" -f 1)
CONTAINER_PORT=$(echo "$CONTAINER_HOST_AND_PORT" | cut -d ":" -f 2)

# Wait until Postgres is ready
RETRY_COUNT=0
RETRY_MAX=10
RETRY_INTERVAL=3
while ! pg_isready --username=$SQL_USER --dbname=$SQL_DATABASE --host=$CONTAINER_HOST --port=$CONTAINER_PORT 2>/dev/null; do
  RETRY_COUNT=$(($RETRY_COUNT + 1))
  if [ $RETRY_COUNT -ge $RETRY_MAX ]; then
    echo "PostgreSQL not ready after ${RETRY_MAX} attempts. Exiting."
    exit 1
  fi
  echo "Waiting for PostgreSQL to be ready... Attempt: ${RETRY_COUNT}"
  sleep "${RETRY_INTERVAL}"
done


# Run the script
export POSTGRES_HOST=$CONTAINER_HOST
export POSTGRES_PORT=$CONTAINER_PORT
export POSTGRES_URL="postgres://$SQL_USER:$SQL_PASSWORD@$CONTAINER_HOST:$CONTAINER_PORT/$SQL_DATABASE"
%ENV_TRANSPOSE%
for binary in %BINARIES%; do
    BINARY_CLI="$(rlocation "$binary")"
    echo "Execute: $binary"
    $BINARY_CLI
done
