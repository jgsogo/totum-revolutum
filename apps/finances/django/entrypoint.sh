#!/bin/sh

if [ "$DATABASE" = "postgres" ]
then
    echo "Waiting for postgres..."

    while ! nc -z $SQL_HOST $SQL_PORT; do
      sleep 0.1
    done

    echo "PostgreSQL started"
fi

/apps/finances/django/admin flush --no-input
/apps/finances/django/admin migrate

/apps/finances/django/admin runserver 0.0.0.0:8080
