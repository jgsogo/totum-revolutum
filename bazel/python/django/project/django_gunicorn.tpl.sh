#!/usr/bin/env bash
set -o pipefail -o errexit -o nounset


%BASH_RLOCATION_FUNCTION%

runfiles_export_envvars

readonly GUNICORN="$(rlocation "%GUNICORN%")"

# Read .env file if available
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
ENV_FILE=$SCRIPT_DIR/.env
if [ -f ${ENV_FILE} ]; then
    echo "Load environment from '.env' file"
    set -a
    source ${ENV_FILE}
    set +a
fi

# Prepare the environment to run the command
if [[ -z "${GUNICORN_APP_NAME}" ]]; then
    >&2 echo "Envvar 'GUNICORN_APP_NAME' is required"
fi

if [[ -z "${GUNICORN_RUN_FOLDER}" ]]; then
    >&2 echo "Envvar 'GUNICORN_RUN_FOLDER' is required"
fi

if [[ -z "${GUNICORN_LOGS_FOLDER}" ]]; then
    >&2 echo "Envvar 'GUNICORN_LOGS_FOLDER' is required"
fi

# Create directories if needed
test -d $GUNICORN_RUN_FOLDER || mkdir -p $GUNICORN_RUN_FOLDER
test -d $GUNICORN_LOGS_FOLDER || mkdir -p $GUNICORN_LOGS_FOLDER

BIND_SOCKET="unix:${GUNICORN_RUN_FOLDER}/gunicorn.sock"
BIND=${GUNICORN_BIND:-$BIND_SOCKET}
echo " - binding to: $BIND"

# Execute gunicorn
DJANGO_SETTINGS_MODULE="%SETTINGS%" $GUNICORN bazel.python.django.project.wsgi:application \
    --name $GUNICORN_APP_NAME \
    --user=${GUNICORN_USER:-jgsogo} --group=${GUNICORN_GROUP:-staff} \
    --bind $BIND \
    --workers ${GUNICORN_WORKERS:-3} \
    --log-level ${GUNICORN_LOG_LEVEL:-info} \
    --access-logfile ${GUNICORN_LOGS_FOLDER}/gunicorn-access.log \
    --error-logfile ${GUNICORN_LOGS_FOLDER}/gunicorn-errors.log \
    ${GUNICORN_EXTRA_ARGS:=}
