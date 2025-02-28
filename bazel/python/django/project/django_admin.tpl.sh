#!/usr/bin/env bash
set -o pipefail -o errexit -o nounset


%BASH_RLOCATION_FUNCTION%

runfiles_export_envvars

readonly DJANGO_ADMIN="$(rlocation "%DJANGO_ADMIN%")"


DJANGO_SETTINGS_MODULE="%SETTINGS%" $DJANGO_ADMIN "$@"
