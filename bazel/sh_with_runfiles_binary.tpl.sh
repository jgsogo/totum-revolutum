#!/usr/bin/env bash
set -o pipefail -o errexit -o nounset

%BASH_RLOCATION_FUNCTION%

runfiles_export_envvars

TOOL_PATH="$(rlocation "%TOOL%")"
if [[ ! -f "${TOOL_PATH:-}" ]]; then
  echo >&2 "ERROR: could not look up the tool path"
  exit 1
fi
echo "TOOL_PATH found at '$TOOL_PATH'"

# Read .env file if available
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
ENV_FILE=$SCRIPT_DIR/.env
if [ -f ${ENV_FILE} ]; then
    echo "Load environment from '.env' file"
    set -a
    source ${ENV_FILE}
    set +a
fi

$TOOL_PATH %TOOL_ARGS% &  # Start task in the background
pid=$!                    # Capture the background process ID

# Cleanup function to stop the background process gracefully
cleanup() {
    echo "Cleaning up..."
    kill $pid 2>/dev/null  # Terminate the background process
    wait $pid 2>/dev/null  # Ensure it fully stops
    echo "Shutdown complete."
    exit 0
}

# Trap multiple signals and call cleanup
trap cleanup SIGINT SIGTERM SIGHUP

# Wait for the background process to finish
wait $pid
