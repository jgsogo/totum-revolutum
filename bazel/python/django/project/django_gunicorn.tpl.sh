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

if [[ -z "${GUNICORN_WORKING_DIR}" ]]; then
    >&2 echo "Envvar 'GUNICORN_WORKING_DIR' is required"
fi

# Create directories if needed
echo "Working directory: $GUNICORN_WORKING_DIR"
test -d $GUNICORN_WORKING_DIR || mkdir -p $GUNICORN_WORKING_DIR
test -d $GUNICORN_WORKING_DIR/run || mkdir -p $GUNICORN_WORKING_DIR/run
test -d $GUNICORN_WORKING_DIR/log || mkdir -p $GUNICORN_WORKING_DIR/log

BIND_SOCKET="unix:${GUNICORN_WORKING_DIR}/run/gunicorn.sock"
BIND=${GUNICORN_BIND:-$BIND_SOCKET}
echo " - binding to: $BIND"

# Execute gunicorn
DJANGO_SETTINGS_MODULE="%SETTINGS%" $GUNICORN bazel.python.django.project.wsgi:application \
    --name $GUNICORN_APP_NAME \
    --user=${GUNICORN_USER:-jgsogo} --group=${GUNICORN_GROUP:-staff} \
    --bind $BIND \
    --workers ${GUNICORN_WORKERS:-3} \
    --log-level ${GUNICORN_LOG_LEVEL:-info} \
    --access-logfile ${GUNICORN_WORKING_DIR}/log/gunicorn-access.log \
    --error-logfile ${GUNICORN_WORKING_DIR}/log/gunicorn-errors.log \
    ${GUNICORN_EXTRA_ARGS:=}
