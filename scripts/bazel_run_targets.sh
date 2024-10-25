#!/bin/sh

# Run all the '$1' targets

targets=$(bazel query "attr('tags', '$1', '//...')")

while IFS= read -r line; do
    bazel run $line
done <<< "$targets"
