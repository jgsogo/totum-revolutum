#!/bin/sh

if [ "$DATABASE" = "postgres" ]
then
    echo "Waiting for postgres..."

    while ! nc -z $SQL_HOST $SQL_PORT; do
      sleep 0.1
    done

    echo "PostgreSQL started"
fi

/apps/finances/django/app-admin flush --no-input
/apps/finances/django/app-admin migrate
/apps/finances/django/app-admin collectstatic --no-input --clear

/apps/finances/django/app-gunicorn wsgi:application --bind 0.0.0.0:8000
