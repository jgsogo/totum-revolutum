#!/usr/bin/env bash
set -o pipefail -o errexit -o nounset


%BASH_RLOCATION_FUNCTION%

runfiles_export_envvars

readonly ENVSUBST="$(rlocation "%ENVSUBST%")"
readonly CONFIG="$(rlocation "%CONFIG%")"
readonly BINARY="$(rlocation "%BINARY%")"

# Generate the config file to use and pipe it to the binary
$ENVSUBST -no-unset < $CONFIG | $ENVSUBST -no-unset | $BINARY
