#!/usr/bin/env bash
set -o pipefail -o errexit -o nounset

%BASH_RLOCATION_FUNCTION%

runfiles_export_envvars

DIESEL_CLI_RPATH="$(rlocation "%DIESEL_CLI%")"
$DIESEL_CLI_RPATH print-schema %DIESEL_CLI_ARGS% --database-url $POSTGRES_URL > %SCHEMA_FILE%
