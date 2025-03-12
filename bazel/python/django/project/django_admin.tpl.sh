#!/usr/bin/env bash
set -o pipefail -o errexit -o nounset


%BASH_RLOCATION_FUNCTION%

runfiles_export_envvars

readonly DJANGO_ADMIN="$(rlocation "%DJANGO_ADMIN%")"

# Read .env file if available
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
ENV_FILE=$SCRIPT_DIR/.env
if [ -f ${ENV_FILE} ]; then
    echo "Load environment from '.env' file"
    set -a
    source ${ENV_FILE}
    set +a
fi

# Execute the command
DJANGO_SETTINGS_MODULE="%SETTINGS%" $DJANGO_ADMIN "$@"
