#!/usr/bin/env bash

set -ex

function finish {
    kill $frontend_pid
}
trap finish EXIT

export BAZEL_BINDIR="."
echo "running frontend"
frontend_logfile="frontend-log.tmp"
bash -c "%{frontend_executable} 2>&1 | tee $frontend_logfile" &
frontend_pid=($!)

backend_logfile="backend-log.tmp"
bash -c "%{backend_executable}" 2>&1 | tee $backend_logfile
