import os

import django
from django.conf import settings
from django.core.management import execute_from_command_line


def setup_application():
    INSTALLED_APPS = [
        "core.apps.BoardGamesCoreConfig",
    ]

    settings.configure(
        SECRET_KEY="fake-key",
        INSTALLED_APPS=INSTALLED_APPS,
        DEBUG=True,
        DATABASES={
            "default": {
                "ENGINE": os.environ.get("DJANGO_SQL_ENGINE", "django.db.backends.sqlite3"),
                "NAME": os.environ.get("DJANGO_SQL_DATABASE", "db.sqlite3"),
                "USER": os.environ.get("DJANGO_SQL_USER", "user"),
                "PASSWORD": os.environ.get("DJANGO_SQL_PASSWORD", "password"),
                "HOST": os.environ.get("DJANGO_SQL_HOST", "localhost"),
                "PORT": os.environ.get("DJANGO_SQL_PORT", "5432"),
            }
        },
        TIME_ZONE="UTC",
        USE_TZ=True,
    )


if __name__ == "__main__":
    setup_application()
    django.setup()

    execute_from_command_line()
