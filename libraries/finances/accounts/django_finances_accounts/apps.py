from django.apps import AppConfig


class FinancesAccountsConfig(AppConfig):
    default_auto_field = "django.db.models.BigAutoField"
    name = "django_finances_accounts"  # FIXME: Make this 'finances.accounts'
    label = "finances_accounts"
    verbose_name = "Finances accounts"
