#!/usr/bin/env bash

set -ex


export BAZEL_BINDIR="."
export RUST_BACKTRACE=1
export RUST_LOG=debug

# Run the backend detached (after one second)
(sleep 1; %{backend_executable} 2>&1) &

# Run the frontend, blocking call. User will need to Ctrl+C to stop it,
# but this way we guarantee that the server is closed and the port is available
# for future invocation
%{frontend_executable} 2>&1
