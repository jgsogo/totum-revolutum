#!/usr/bin/env bash
set -o pipefail -o errexit -o nounset

%BASH_RLOCATION_FUNCTION%

runfiles_export_envvars

export BAZEL_BINDIR="."
export RUST_BACKTRACE=1
export RUST_LOG=debug

BEFORE_PATH="$(rlocation "%{before_executable}")"
if [[ ! -f "${BEFORE_PATH:-}" ]]; then
  echo >&2 "ERROR: could not look up the before path"
  exit 1
fi

AFTER_PATH="$(rlocation "%{after_executable}")"
if [[ ! -f "${AFTER_PATH:-}" ]]; then
  echo >&2 "ERROR: could not look up the after path"
  exit 1
fi

# Collect the PID of the first executable (detached), so
# we can kill the process when this script finishes
pids=()

function finish {
    for pid in "${pids[@]}"; do
        echo "Kill pid $pid"
        kill $pid
    done
}
trap finish EXIT
trap finish INT  # Capture Ctrl_C (SIGINT)

# Run the 'before' detached
echo "Running first executable '$BEFORE_PATH'"
$BEFORE_PATH 2>&1 &
pid=$!
echo " - started with pid '$pid'"
pids+=($pid)

# Configurable sleep
echo "Sleep for %{sleep} seconds"
sleep %{sleep}

# Run the after, blocking call. User will need to Ctrl+C to stop it
echo "Running after (use Ctrl+C to finish it) '$AFTER_PATH'"
PWD=$(pwd)
echo "PWD: $PWD"
$AFTER_PATH 2>&1
