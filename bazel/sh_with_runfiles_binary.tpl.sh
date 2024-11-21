#!/usr/bin/env bash
set -o pipefail -o errexit -o nounset

%BASH_RLOCATION_FUNCTION%

runfiles_export_envvars

TOOL_PATH="$(rlocation "%TOOL%")"
if [[ ! -f "${TOOL_PATH:-}" ]]; then
  echo >&2 "ERROR: could not look up the tool path"
  exit 1
fi

"$TOOL_PATH" %TOOL_ARGS%
