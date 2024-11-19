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

# Ensure we execute the docker compose DOWN
function finish {
    "$CONTAINER_CLI" stop %CONTAINER_NAME%
}
trap finish EXIT

# Start docker compose UP
"$CONTAINER_CLI" run --rm -p %INNER_PORT% --name=%CONTAINER_NAME% --env-file=%ENV_FILE% -d %IMAGE%
CONTAINER_PORT=$("$CONTAINER_CLI" port %CONTAINER_NAME% %INNER_PORT%)

echo "external port: $CONTAINER_PORT"

%LIVENESS_PROBE%

# Run the script
%CMD%
