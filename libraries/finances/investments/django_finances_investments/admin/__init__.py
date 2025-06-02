from django.contrib import admin
from django_finances_accounts.admin.inlines import (
    AccountHolderRoleInline,
    MovementInInline,
    MovementOutInline,
    SnapshotInline,
)
from django_finances_accounts.models import Account, Transaction

from ..models import MovementDividend, MovementNumerable, SnapshotNumerable
from ._filter_numerable_accounts import _FilterNonNumerableAccounts
from .movement_dividend import (
    MovementDividendAdmin,
    MovementDividendInInline,
    MovementDividendOutInline,
)
from .movement_numerable import (
    MovementNumerableAdmin,
    MovementNumerableInInline,
    MovementNumerableOutInline,
)
from .snapshot_numerable import SnapshotNumerableInline, SnapshotNumerableModelAdmin

admin.site.register(SnapshotNumerable, SnapshotNumerableModelAdmin)
admin.site.register(MovementNumerable, MovementNumerableAdmin)
admin.site.register(MovementDividend, MovementDividendAdmin)


# Append more inlines to models from finances/accounts (taken from
# https://stackoverflow.com/questions/32590901/how-can-i-add-inlines-to-the-modeladmin-of-another-app-without-a-circular-depen)


class MovementInInline(_FilterNonNumerableAccounts, MovementInInline):
    def get_queryset(self, request):
        qs = super().get_queryset(request)
        # Filter out all snapshots that are MovementNumerable
        qs = qs.exclude(pk__in=MovementNumerable.objects.all().values_list("pk", flat=True))
        return qs


class MovementOutInline(_FilterNonNumerableAccounts, MovementOutInline):
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


class SnapshotInline(_FilterNonNumerableAccounts, SnapshotInline):

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
