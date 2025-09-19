#!/usr/bin/env bash
set -o pipefail -o errexit -o nounset

%BASH_RLOCATION_FUNCTION%

runfiles_export_envvars

DJANGO_ADMIN="$(rlocation "%DJANGO_ADMIN%")"

# --noinput             Tells Django to NOT prompt the user for input of any kind.
# --check               Exit with a non-zero status if model changes are missing migrations and don't actually write them. Implies --dry-
#                       run.
# --scriptable          Divert log output and input prompts to stderr, writing only paths of generated migration files to stdout.
set +e
"$DJANGO_ADMIN" makemigrations --check %APP_LABEL%
RESULT=$?
set -e

if [ $RESULT == 0 ] ; then
    exit $RESULT
else
    echo "Use 'bazel run %UPDATE_RULE%' to create pending migrations"
    exit $RESULT
fi
