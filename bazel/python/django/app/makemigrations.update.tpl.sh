#!/usr/bin/env bash
set -o pipefail -o errexit -o nounset

%BASH_RLOCATION_FUNCTION%

runfiles_export_envvars

DJANGO_ADMIN="$(rlocation "%DJANGO_ADMIN%")"

# --noinput             Tells Django to NOT prompt the user for input of any kind.
# --check               Exit with a non-zero status if model changes are missing migrations and don't actually write them. Implies --dry-
#                       run.
# --scriptable          Divert log output and input prompts to stderr, writing only paths of generated migration files to stdout.
migration_file=$($DJANGO_ADMIN makemigrations --scriptable %APP_LABEL%)

if [ -z "$migration_file" ]
then
    echo "Nothing to migrate. Everything up to date"
else
    echo "Created migration: $migration_file"
    cp -fv $(pwd)/$migration_file $BUILD_WORKSPACE_DIRECTORY/$migration_file
fi
