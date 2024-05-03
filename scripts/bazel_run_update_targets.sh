#!/bin/sh

# Run all the 'update' targets

targets=$(bazel query "attr('tags', 'update', '//...')")

while IFS= read -r line; do
    bazel run $line
done <<< "$targets"
