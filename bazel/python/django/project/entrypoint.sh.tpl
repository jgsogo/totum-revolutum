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

# DJANGO_SUPERUSER_PASSWORD=django /apps/finances/django/app-admin createsuperuser --email=superuser@app.com --username=django --noinput

/apps/finances/django/app-gunicorn bazel.python.django.project.wsgi:application --bind 0.0.0.0:%DJANGO_PORT%
