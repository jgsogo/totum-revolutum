from django.contrib import admin
from django_finances_accounts.admin.inlines import get_movement_inline_form
from django_finances_accounts.models import MovementDirection
from django_finances_investments.models import (
    MovementNumerable,
)

from ._filter_numerable_accounts import _FilterNumerableAccounts


class MovementNumerableAdmin(_FilterNumerableAccounts, admin.ModelAdmin):
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
    readonly_fields = ("amount",)


class MovementNumerableInInline(_FilterNumerableAccounts, admin.TabularInline):
    model = MovementNumerable
    form = get_movement_inline_form(MovementDirection.IN)
    exclude = ("direction",)
    readonly_fields = ("amount",)
    extra = 1

    verbose_name = "Movement Numerable IN"
    verbose_name_plural = "Movements Numerable IN"

    def get_queryset(self, request):
        qs = super().get_queryset(request)
        return qs.filter(direction=MovementDirection.IN)


class MovementNumerableOutInline(_FilterNumerableAccounts, admin.TabularInline):
    model = MovementNumerable
    form = get_movement_inline_form(MovementDirection.IN)
    exclude = ("direction",)
    readonly_fields = ("amount",)
    extra = 1

    verbose_name = "Movement Numerable OUT"
    verbose_name_plural = "Movements Numerable OUT"

    def get_queryset(self, request):
        qs = super().get_queryset(request)
        return qs.filter(direction=MovementDirection.OUT)
