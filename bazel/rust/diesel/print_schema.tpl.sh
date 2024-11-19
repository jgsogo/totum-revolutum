#!/usr/bin/env bash
set -o pipefail -o errexit -o nounset

%BASH_RLOCATION_FUNCTION%

runfiles_export_envvars

DIESEL_CLI_RPATH="$(rlocation "%DIESEL_CLI%")"

pg_isready --username=$POSTGRES_USER --dbname=$POSTGRES_DB --host=$POSTGRES_HOST --port=$POSTGRES_PORT
psql $POSTGRES_URL -c "SELECT datname FROM pg_database WHERE datistemplate = false;";
all_tables="""
SELECT
    table_schema || '.' || table_name
FROM
    information_schema.tables
WHERE
    table_type = 'BASE TABLE'
AND
    table_schema NOT IN ('pg_catalog', 'information_schema');
"""
psql $POSTGRES_URL -c "$all_tables";

echo '>>>>> Diesel print-schema'
echo "Args: print-schema --database-url $POSTGRES_URL > $BUILD_WORKSPACE_DIRECTORY/schema.postgres.rs"

$DIESEL_CLI_RPATH print-schema --database-url $POSTGRES_URL
$DIESEL_CLI_RPATH print-schema --database-url $POSTGRES_URL > $BUILD_WORKSPACE_DIRECTORY/schema.postgres.rs
