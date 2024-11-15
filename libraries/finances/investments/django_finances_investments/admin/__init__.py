from django.contrib import admin

from ..models import MovementDividend, MovementNumerable, SnapshotNumerable


class SnapshotNumerableModelAdmin(admin.ModelAdmin):
    list_display = ("account", "date_value", "quantity", "unit_value")
    list_filter = ("account", "date_value")


admin.site.register(SnapshotNumerable, SnapshotNumerableModelAdmin)
admin.site.register(MovementNumerable)
admin.site.register(MovementDividend)
