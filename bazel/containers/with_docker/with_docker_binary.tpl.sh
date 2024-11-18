#!/usr/bin/env bash
set -o pipefail -o errexit -o nounset


{{BASH_RLOCATION_FUNCTION}}

runfiles_export_envvars

readonly DOCKER_CLI="$(rlocation "{{docker_cli}}")"
# readonly DOCKER_CLI="{{docker_cli}}"

echo "DOCKER_CLI: $DOCKER_CLI"

if [ -f "$DOCKER_CLI" ]; then
    CONTAINER_CLI="$DOCKER_CLI"
elif command -v docker &> /dev/null; then
    CONTAINER_CLI="docker"
elif command -v podman &> /dev/null; then
    CONTAINER_CLI="podman"
# elif command -v /Users/jgsogo/.rd/bin/docker &> /dev/null; then #FIXME: This path is a bit hardcoded!
#     CONTAINER_CLI="/Users/jgsogo/.rd/bin/docker"
else
    echo >&2 "Neither docker or podman could be found."
    echo >&2 "To use a different container runtime, pass an executable to the 'docker_cli' attribute."
    exit 1
fi

echo ">>>> CONTAINER_CLI: $CONTAINER_CLI"

# Ensure we execute the docker compose DOWN
function finish {
    echo ">>>> Stop and remove the docker compose (project name %PROJECT_NAME%)"
    "$CONTAINER_CLI" compose --project-name %PROJECT_NAME% --env-file %ENV_FILE% down --volumes --timeout=30
}
trap finish EXIT

# Start docker compose UP
echo ">>>> Run the docker compose (project name %PROJECT_NAME%)"
"$CONTAINER_CLI" compose --project-name %PROJECT_NAME% --env-file %ENV_FILE% up %SERVICES% --wait

# Run the script
echo ">>>> Just sleep for a few seconds"
# sleep 5
pg_isready --username=$SQL_USER --dbname=$SQL_DATABASE --host=127.0.0.1 --port=5431

pwd
echo ">>>> CMD: %CMD%"
%CMD%

# binary=$(rlocation %CMD%)
# echo "binary: $binary"
# $binary
# binary_path="$(rlocation %BINARY%)"
# $binary_path
