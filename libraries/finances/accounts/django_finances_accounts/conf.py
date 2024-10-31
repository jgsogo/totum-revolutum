from appconf import AppConf
from django.conf import settings  # noqa: F401


class FinancesAccountsConf(AppConf):
    MONEY_TOLERANCE = 0

    # If set to True, only the AccountTypes that are used to run
    # some business logic will be created.
    MIGRATE_ONLY_REQUIRED_ACCOUNTTYPES = False

    # If set to True, only the MovementTypes that are used to run
    # some business logic will be created.
    MIGRATE_ONLY_REQUIRED_MOVMENTTYPES = False

    class Meta:
        prefix = "finances"
