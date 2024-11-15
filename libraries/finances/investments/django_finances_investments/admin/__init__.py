from django.contrib import admin

from ..models import MovementDividend, MovementNumerable, SnapshotNumerable


class SnapshotNumerableModelAdmin(admin.ModelAdmin):
    list_display = ("account", "date_value", "quantity", "unit_value")
    list_filter = ("account", "date_value")


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
    search_fields = ("transaction__group",)


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
    search_fields = ("transaction__group",)


admin.site.register(MovementDividend, MovementDividendAdmin)
