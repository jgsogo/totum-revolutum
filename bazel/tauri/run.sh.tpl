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

# Run the backend detached (after one second)
(sleep 1; $BACKEND_PATH 2>&1) &

# Run the frontend, blocking call. User will need to Ctrl+C to stop it,
# but this way we guarantee that the server is closed and the port is available
# for future invocation
$FRONTEND_PATH 2>&1
