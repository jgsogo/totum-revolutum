import django
from django.conf import settings
from django.core.management import execute_from_command_line

# BASE_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), "your_app"))


def setup_application():
    settings.configure(
        # BASE_DIR=BASE_DIR,
        SECRET_KEY="fake-key",
        INSTALLED_APPS=[
            "tests",
            "django_finances_accounts.apps.FinancesAccountsConfig",
            "treenode",
            "djmoney",
            "django_countries",
        ],
        DEBUG=True,
        DATABASES={
            "default": {
                "ENGINE": "django.db.backends.sqlite3",
                "NAME": "db.sqlite3",
                # "NAME": os.path.join(BASE_DIR, "db.sqlite3"),
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

    django.setup()


if __name__ == "__main__":
    # os.environ["DJANGO_SETTINGS_MODULE"] = "tests.test_settings"
    # django.setup()
    setup_application()
    execute_from_command_line()

    # TestRunner = get_runner(settings)
    # test_runner = TestRunner()
    # failures = test_runner.run_tests(["tests"])
    # sys.exit(bool(failures))
