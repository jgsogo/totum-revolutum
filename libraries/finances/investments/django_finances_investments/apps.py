from django.apps import AppConfig


class FinancesInvestmentsConfig(AppConfig):
    default_auto_field = "django.db.models.BigAutoField"
    name = "django_finances_investments"  # FIXME: Make this 'finances.stocks'
    label = "finances_investments"
    verbose_name = "Finances investments"
