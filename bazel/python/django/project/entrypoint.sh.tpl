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


/apps/finances/django/app-admin migrate
/apps/finances/django/app-admin collectstatic --no-input --clear

if [ -v "${DJANGO_SUPERUSER_PASSWORD}" ]; then
    # If the username is already taken, this command will fail, but the script will continue
    echo "Creating superuser '$DJANGO_SUPERUSER_USERNAME'"
    /apps/finances/django/app-admin createsuperuser --noinput
fi

/apps/finances/django/app-gunicorn
