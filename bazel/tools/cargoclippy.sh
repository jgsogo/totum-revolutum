#!/usr/bin/env bash

set -eu
set -x

bazel run @rules_rust//tools/upstream_wrapper:cargo_clippy
