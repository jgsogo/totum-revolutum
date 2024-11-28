#!/bin/sh

# Run all the '$1' targets

targets=$(bazel query 'kind("oci_load", //...)')

while IFS= read -r line; do
    bazel run $line
done <<< "$targets"
