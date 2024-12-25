#!/usr/bin/env bash
set -o pipefail -o errexit -o nounset

%BASH_RLOCATION_FUNCTION%

runfiles_export_envvars


export BAZEL_BINDIR="."
export RUST_BACKTRACE=1
export RUST_LOG=debug

BACKEND_PATH="$(rlocation "%{backend_executable}")"
if [[ ! -f "${BACKEND_PATH:-}" ]]; then
  echo >&2 "ERROR: could not look up the backend path"
  exit 1
fi

FRONTEND_PATH="$(rlocation "%{frontend_executable}")"
if [[ ! -f "${FRONTEND_PATH:-}" ]]; then
  echo >&2 "ERROR: could not look up the frontend path"
  exit 1
fi

pids=()

function finish {
    for pid in "${pids[@]}"; do
        echo "Kill pid $pid"
        kill $pid
    done
}
trap finish EXIT
trap finish INT  # Capture Ctrl_C (SIGINT)

# Run the frontend detached (after one second)
echo "Running backend '$FRONTEND_PATH'"
$FRONTEND_PATH 2>&1 &
pid=$!
echo " - started with pid '$pid'"
pids+=($pid)

# Run the backend, blocking call. User will need to Ctrl+C to stop it,
# but this way we guarantee that the server is closed and the port is available
# for future invocation
echo "Running backend (after one second sleep) '$BACKEND_PATH'"
sleep 1
$BACKEND_PATH 2>&1
