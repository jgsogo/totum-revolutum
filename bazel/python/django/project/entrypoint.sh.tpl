#!/bin/sh

if [ "$DATABASE" = "postgres" ]
then
    echo "Waiting for postgres..."

    while ! nc -z $SQL_HOST $SQL_PORT; do
      sleep 0.1
    done

    echo "PostgreSQL started"
fi

/apps/finances/django/app-admin migrate
/apps/finances/django/app-admin collectstatic --no-input --clear

if [ -n "${DJANGO_SUPERUSER_PASSWORD}" ]; then
    # If the username is already taken, this command will fail, but the script will continue
    echo "Creating superuser '$DJANGO_SUPERUSER_USERNAME'"
    /apps/finances/django/app-admin createsuperuser --noinput
fi

/apps/finances/django/app-gunicorn
