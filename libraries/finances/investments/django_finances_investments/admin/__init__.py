from django.contrib import admin

from ..models import MovementDividend, MovementNumerable, SnapshotNumerable

# from django_finances_accounts.models import Transaction, Account
# from .inlines import (
#     MovementNumerableInInline,
#     MovementNumerableOutInline,
#     MovementDividendInInline,
#     MovementDividendOutInline,
#     SnapshotNumerableInline,
# )

# Append more inlines to models from finances/accounts
# (taken from https://stackoverflow.com/questions/32590901/how-can-i-add-inlines-to-the-modeladmin-of-another-app-without-a-circular-depen)
# admin.site._registry[Transaction].inlines.extend([MovementNumerableInInline, MovementNumerableOutInline, MovementDividendInInline, MovementDividendOutInline])
# admin.site._registry[Account].inlines.append(SnapshotNumerableInline)
# FIXME: Remove snapshots/movements that are instances of these child classes. We want them to show up in these inlines, not the parent's


class SnapshotNumerableModelAdmin(admin.ModelAdmin):
    list_display = ("account", "date_value", "quantity", "unit_value")
    list_filter = ("account", "date_value")
    search_fields = ("account",)


admin.site.register(SnapshotNumerable, SnapshotNumerableModelAdmin)


class MovementNumerableAdmin(admin.ModelAdmin):
    list_display = (
        "account",
        "date_value",
        "direction",
        "transaction__group",
        "type",
        "quantity",
        "unit_value",
    )
    list_filter = (
        "date_value",
        "direction",
        "type",
    )
    search_fields = ("transaction__group", "account", "transaction")


admin.site.register(MovementNumerable, MovementNumerableAdmin)


class MovementDividendAdmin(admin.ModelAdmin):
    list_display = (
        "account",
        "date_value",
        "direction",
        "transaction__group",
        "type",
        "ex_dividend_date",
        "unit_value",
    )
    list_filter = (
        "ex_dividend_date",
        "date_value",
        "direction",
        "type",
    )
    search_fields = ("transaction__group", "account", "transaction")


admin.site.register(MovementDividend, MovementDividendAdmin)
