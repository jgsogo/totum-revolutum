import django
from django.conf import settings
from django.core.management import execute_from_command_line


def setup_application():
    INSTALLED_APPS = [
        "django_finances_investments.apps.FinancesInvestmentsConfig",
        # deps
        "django_finances_accounts.apps.FinancesAccountsConfig",
        "treenode",
        "djmoney",
        "django_countries",
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
        # Configure django-money
        BASE_CURRENCY="EUR",
        CURRENCIES=("EUR", "USD"),
        # Configure django-countries
        COUNTRIES_ONLY=["ES", "US", "NL"],
    )


if __name__ == "__main__":
    setup_application()
    django.setup()

    execute_from_command_line()
