#!/bin/sh

set -eux pipefail

# Wait until Postgres is ready
RETRY_COUNT=0
RETRY_MAX=10
RETRY_INTERVAL=3
while ! pg_isready --username=$DJANGO_SQL_USER --dbname=$DJANGO_SQL_DATABASE --host=$DJANGO_SQL_HOST --port=$DJANGO_SQL_PORT 2>/dev/null; do
RETRY_COUNT=$(($RETRY_COUNT + 1))
if [ $RETRY_COUNT -ge $RETRY_MAX ]; then
    echo "PostgreSQL not ready after ${RETRY_MAX} attempts. Exiting."
    exit 1
fi
echo "Waiting for PostgreSQL to be ready... Attempt: ${RETRY_COUNT}"
sleep "${RETRY_INTERVAL}"
done

# If provided, it will copy the production database
if [ -n "$PRODUCTION_DATABASE_URL" ]; then
    echo "Copying content from production database (only if empty)"
    DEV_DATABASE="postgres://$DJANGO_SQL_USER:$DJANGO_SQL_PASSWORD@$DJANGO_SQL_HOST:$DJANGO_SQL_PORT/$DJANGO_SQL_DATABASE"

    # Check for user-defined tables in public schema
    echo "Check if dev database is empty ($DEV_DATABASE)"
    TABLE_COUNT=$(psql -d $DEV_DATABASE -Atc "
        SELECT COUNT(*) FROM information_schema.tables
        WHERE table_schema = 'public' AND table_type = 'BASE TABLE';
    ")
    echo "Table count ($TABLE_COUNT)"

    if [ "$TABLE_COUNT" -eq 0 ]; then
        echo "Database '$DJANGO_SQL_DATABASE' is empty (no user-defined tables)."
        pg_dump --no-privileges --no-owner -d $PRODUCTION_DATABASE_URL | psql -d postgres://$DJANGO_SQL_USER:$DJANGO_SQL_PASSWORD@$DJANGO_SQL_HOST:$DJANGO_SQL_PORT/$DJANGO_SQL_DATABASE
    else
        echo "Database '$DJANGO_SQL_DATABASE' is NOT empty. Table count: $TABLE_COUNT"
    fi
fi

/apps/finances/django/app-admin migrate

/apps/finances/django/app-admin collectstatic --no-input --clear

if [ -n "$DJANGO_SUPERUSER_PASSWORD" ]; then
    # If the username is already taken, this command will fail, but the script will continue
    echo "Creating superuser '$DJANGO_SUPERUSER_USERNAME'"
    /apps/finances/django/app-admin createsuperuser --noinput
fi

/apps/finances/django/app-gunicorn
