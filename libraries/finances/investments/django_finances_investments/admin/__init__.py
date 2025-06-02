from django.contrib import admin
from django_finances_accounts.admin.inlines import (
    AccountHolderRoleInline,
    MovementInInline,
    MovementOutInline,
    SnapshotInline,
)
from django_finances_accounts.models import Account, Transaction

from ..models import MovementDividend, MovementNumerable, SnapshotNumerable
from .inlines import (
    MovementDividendInInline,
    MovementDividendOutInline,
    MovementNumerableInInline,
    MovementNumerableOutInline,
)
from .snapshot_numerable import SnapshotNumerableInline, SnapshotNumerableModelAdmin

# Append more inlines to models from finances/accounts (taken from
# https://stackoverflow.com/questions/32590901/how-can-i-add-inlines-to-the-modeladmin-of-another-app-without-a-circular-depen)


class MovementInInline(MovementInInline):
    def get_queryset(self, request):
        qs = super().get_queryset(request)
        # Filter out all snapshots that are MovementNumerable
        qs = qs.exclude(pk__in=MovementNumerable.objects.all().values_list("pk", flat=True))
        return qs


class MovementOutInline(MovementOutInline):
    def get_queryset(self, request):
        qs = super().get_queryset(request)
        # Filter out all snapshots that are MovementNumerable
        qs = qs.exclude(pk__in=MovementNumerable.objects.all().values_list("pk", flat=True))
        return qs


admin.site._registry[Transaction].inlines = [
    MovementInInline,
    MovementOutInline,
    MovementNumerableInInline,
    MovementNumerableOutInline,
    MovementDividendInInline,
    MovementDividendOutInline,
]


class SnapshotInline(SnapshotInline):

    def get_queryset(self, request):
        qs = super().get_queryset(request)
        # Filter out all snapshots that are SnapshotNumerable
        qs = qs.exclude(pk__in=SnapshotNumerable.objects.all().values_list("pk", flat=True))
        return qs


admin.site._registry[Account].inlines = [
    AccountHolderRoleInline,
    SnapshotInline,
    SnapshotNumerableInline,
]


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
    readonly_fields = ("amount",)


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
    readonly_fields = ("amount",)


admin.site.register(MovementDividend, MovementDividendAdmin)
