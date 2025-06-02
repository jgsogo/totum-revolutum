from django.contrib import admin
from django_finances_accounts.models import Account
from django_finances_investments.models import SnapshotNumerable


class _FilterNumerableAccounts:
    def formfield_for_foreignkey(self, db_field, request, **kwargs):
        if db_field.name == "account":
            kwargs["queryset"] = Account.objects.filter(is_numerable=True)
        return super().formfield_for_foreignkey(db_field, request, **kwargs)


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
