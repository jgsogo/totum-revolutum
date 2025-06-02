from django.contrib import admin
from django_finances_investments.models import SnapshotNumerable

from ._filter_numerable_accounts import _FilterNumerableAccounts


class SnapshotNumerableInline(_FilterNumerableAccounts, admin.TabularInline):
    model = SnapshotNumerable
    readonly_fields = ("amount",)
    extra = 1

    verbose_name = "Snapshot numerable"
    verbose_name_plural = "Snapshots numerable"


class SnapshotNumerableModelAdmin(_FilterNumerableAccounts, admin.ModelAdmin):
    list_display = ("account", "date_value", "quantity", "unit_value")
    list_filter = ("account", "date_value")
    search_fields = ("account",)
    readonly_fields = ("amount",)
