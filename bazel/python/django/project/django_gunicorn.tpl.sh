#!/usr/bin/env bash
set -o pipefail -o errexit -o nounset


%BASH_RLOCATION_FUNCTION%

runfiles_export_envvars

readonly GUNICORN="$(rlocation "%GUNICORN%")"

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

# Execute gunicorn
DJANGO_SETTINGS_MODULE="%SETTINGS%" $GUNICORN bazel.python.django.project.wsgi:application \
    --name $GUNICORN_APP_NAME \
    --user=${USER:-jgsogo} --group=${GROUP:-staff} \
    --bind unix:${GUNICORN_WORKING_DIR}/run/gunicorn.sock \
    --workers ${GUNICORN_WORKERS:-3} \
    --log-level ${GUNICORN_LOG_LEVEL:-info} \
    --log-file ${GUNICORN_WORKING_DIR}/log/gunicorn.log
