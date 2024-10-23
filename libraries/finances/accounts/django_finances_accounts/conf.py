from appconf import AppConf
from django.conf import settings  # noqa: F401


class FinancesAccountsConf(AppConf):
    # finances-data
    MONEY_TOLERANCE = 0

    # # Configure django-money
    # BASE_CURRENCY = "EUR"
    # CURRENCIES = ("EUR", "USD")

    # # TODO: Add exchange rates (and near-time): https://stackoverflow.com/questions/1139393/what-is-the-best-django-model-field-to-use-to-represent-a-us-dollar-amount  # noqa: E501

    # # Configure django-countries
    # COUNTRIES_ONLY = ["ES", "US", "NL"]

    class Meta:
        prefix = "finances"
