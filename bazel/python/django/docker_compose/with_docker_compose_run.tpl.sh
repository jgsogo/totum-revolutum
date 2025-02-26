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
    $DOCKER_COMPOSE_COMMAND down %DOCKER_COMPOSE_DOWN_ARGS%
}
trap finish EXIT SIGTERM SIGINT  # Capture Ctrl_C (SIGINT)

# Start docker compose UP
# $DOCKER_COMPOSE_COMMAND config %SERVICES%
$DOCKER_COMPOSE_COMMAND up %SERVICES% --wait --wait-timeout 120
# $DOCKER_COMPOSE_COMMAND up %SERVICES%

# Get the external port for internal 5432
SERVICE_NAME="db"
SERVICE_PORT=5432
POSTGRES_HOST_AND_PORT=$($DOCKER_COMPOSE_COMMAND port db 5432 | head -n 1)
export POSTGRES_HOST=$(echo "$POSTGRES_HOST_AND_PORT" | cut -d ":" -f 1)
export POSTGRES_PORT=$(echo "$POSTGRES_HOST_AND_PORT" | cut -d ":" -f 2)

DJANGO_HOST_AND_PORT=$($DOCKER_COMPOSE_COMMAND port nginx 80 | head -n 1)
export DJANGO_HOST=$(echo "$DJANGO_HOST_AND_PORT" | cut -d ":" -f 1)
export DJANGO_PORT=$(echo "$DJANGO_HOST_AND_PORT" | cut -d ":" -f 2)
export DJANGO_BASE_URL="http://$DJANGO_HOST:$DJANGO_PORT"

# Wait until Postgres is ready
RETRY_COUNT=0
RETRY_MAX=10
RETRY_INTERVAL=3
while ! pg_isready --username=$SQL_USER --dbname=$SQL_DATABASE --host=$POSTGRES_HOST --port=$POSTGRES_PORT 2>/dev/null; do
  RETRY_COUNT=$(($RETRY_COUNT + 1))
  if [ $RETRY_COUNT -ge $RETRY_MAX ]; then
    echo "PostgreSQL not ready after ${RETRY_MAX} attempts. Exiting."
    exit 1
  fi
  echo "Waiting for PostgreSQL to be ready... Attempt: ${RETRY_COUNT}"
  sleep "${RETRY_INTERVAL}"
done


# Run the script
export POSTGRES_URL="postgres://$SQL_USER:$SQL_PASSWORD@$POSTGRES_HOST:$POSTGRES_PORT/$SQL_DATABASE"
%ENV_TRANSPOSE%
for binary in %BINARIES%; do
    BINARY_CLI="$(rlocation "$binary")"
    echo "Execute: $binary"
    $BINARY_CLI
done
