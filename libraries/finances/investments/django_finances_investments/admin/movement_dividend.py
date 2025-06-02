from django.contrib import admin
from django_finances_accounts.admin.inlines import get_movement_inline_form
from django_finances_accounts.models import MovementDirection
from django_finances_investments.models import (
    MovementDividend,
)

from ._filter_numerable_accounts import _FilterNumerableAccounts


class MovementDividendAdmin(_FilterNumerableAccounts, admin.ModelAdmin):
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
    readonly_fields = ("amount",)


class MovementDividendInInline(_FilterNumerableAccounts, admin.TabularInline):
    model = MovementDividend
    form = get_movement_inline_form(MovementDirection.IN)
    exclude = ("direction",)
    readonly_fields = ("amount",)
    extra = 1

    verbose_name = "Movement Dividend IN"
    verbose_name_plural = "Movements Dividend IN"

    def get_queryset(self, request):
        qs = super().get_queryset(request)
        return qs.filter(direction=MovementDirection.IN)


class MovementDividendOutInline(_FilterNumerableAccounts, admin.TabularInline):
    model = MovementDividend
    form = get_movement_inline_form(MovementDirection.IN)
    exclude = ("direction",)
    readonly_fields = ("amount",)
    extra = 1

    verbose_name = "Movement Dividend OUT"
    verbose_name_plural = "Movements Dividend OUT"

    def get_queryset(self, request):
        qs = super().get_queryset(request)
        return qs.filter(direction=MovementDirection.OUT)
