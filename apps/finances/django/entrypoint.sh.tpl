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

if [ -n "$LEGACY_DATABASE_URL" ]; then
    /apps/finances/django/app-admin check_movement_types --LEGACY_DATABASE_URL=$LEGACY_DATABASE_URL
    /apps/finances/django/app-admin check_account_types --LEGACY_DATABASE_URL=$LEGACY_DATABASE_URL
    /apps/finances/django/app-admin migrate_legacy_db --LEGACY_DATABASE_URL=$LEGACY_DATABASE_URL
fi

/apps/finances/django/app-admin collectstatic --no-input --clear

if [ -n "$DJANGO_SUPERUSER_PASSWORD" ]; then
    # If the username is already taken, this command will fail, but the script will continue
    echo "Creating superuser '$DJANGO_SUPERUSER_USERNAME'"
    /apps/finances/django/app-admin createsuperuser --noinput
fi

/apps/finances/django/app-gunicorn bazel.python.django.project.wsgi:application --bind 0.0.0.0:%DJANGO_PORT%
