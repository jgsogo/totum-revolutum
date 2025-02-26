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

# Ensure we stop the docker container
function finish {
    "$CONTAINER_CLI" stop %CONTAINER_NAME%
}
trap finish EXIT SIGTERM SIGINT

# Start postgres container
"$CONTAINER_CLI" run --rm -p 5432 --name=%CONTAINER_NAME% --env-file=%ENV_FILE% -d postgres:%POSTGRES_IMAGE_TAG%

# Get the external port for internal 5432
CONTAINER_HOST_AND_PORT=$("$CONTAINER_CLI" port %CONTAINER_NAME% 5432 | head -n 1)
POSTGRES_HOST=$(echo "$CONTAINER_HOST_AND_PORT" | cut -d ":" -f 1)
POSTGRES_PORT=$(echo "$CONTAINER_HOST_AND_PORT" | cut -d ":" -f 2)

# Wait until Postgres is ready
RETRY_COUNT=0
RETRY_MAX=10
RETRY_INTERVAL=3
while ! pg_isready --username=$POSTGRES_USER --dbname=$POSTGRES_DB --host=$POSTGRES_HOST --port=$POSTGRES_PORT 2>/dev/null; do
  RETRY_COUNT=$(($RETRY_COUNT + 1))
  if [ $RETRY_COUNT -ge $RETRY_MAX ]; then
    echo "PostgreSQL not ready after ${RETRY_MAX} attempts. Exiting."
    exit 1
  fi
  echo "Waiting for PostgreSQL to be ready... Attempt: ${RETRY_COUNT}"
  sleep "${RETRY_INTERVAL}"
done


# Populate some common envvars
export POSTGRES_URL="postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@$POSTGRES_HOST:$POSTGRES_PORT/$POSTGRES_DB"
export SQL_DATABASE=$POSTGRES_DB
export SQL_USER=$POSTGRES_USER
export SQL_PASSWORD=$POSTGRES_PASSWORD
export SQL_HOST=$POSTGRES_HOST
export SQL_PORT=$POSTGRES_PORT

%ENV_TRANSPOSE%
# Run the binaries
for binary in %BINARIES%; do
    BINARY_CLI="$(rlocation "$binary")"
    $BINARY_CLI
done
