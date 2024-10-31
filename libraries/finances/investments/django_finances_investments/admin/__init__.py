from django.contrib import admin

from ..models import MovementDividend, MovementNumerable, SnapshotNumerable

admin.site.register(SnapshotNumerable)
admin.site.register(MovementNumerable)
admin.site.register(MovementDividend)
