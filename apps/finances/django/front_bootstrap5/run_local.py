import django
from django.conf import settings
from django.core.management import execute_from_command_line


def setup_application():
    INSTALLED_APPS = [
        "migrate_legacy",
    ]

    settings.configure(
        SECRET_KEY="fake-key",
        INSTALLED_APPS=INSTALLED_APPS,
        DEBUG=True,
        DATABASES={
            "default": {
                "ENGINE": "django.db.backends.sqlite3",
                "NAME": "db.sqlite3",
            }
        },
        TIME_ZONE="UTC",
        USE_TZ=True,
    )


if __name__ == "__main__":
    setup_application()
    django.setup()

    execute_from_command_line()
