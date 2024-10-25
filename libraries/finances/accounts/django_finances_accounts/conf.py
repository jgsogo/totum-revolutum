from appconf import AppConf
from django.conf import settings  # noqa: F401


class FinancesAccountsConf(AppConf):
    MONEY_TOLERANCE = 0

    class Meta:
        prefix = "finances"
