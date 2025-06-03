#!/bin/sh

set -euox pipefail

targets=$(bazel query 'kind("oci_load", //...)')

while IFS= read -r line; do
    bazel run $line
done <<< "$targets"
