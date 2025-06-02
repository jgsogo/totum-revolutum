from django.contrib import admin
from django_finances_accounts.models import Movement, MovementDirection, Snapshot
from django_finances_accounts.models.account import AccountHolderRole


class MovementInInline(admin.TabularInline):
    model = Movement
    extra = 1

    verbose_name = "Movement IN"
    verbose_name_plural = "Movements IN"

    def get_queryset(self, request):
        qs = super().get_queryset(request)
        return qs.filter(direction=MovementDirection.IN)


class MovementOutInline(admin.TabularInline):
    model = Movement
    extra = 1

    verbose_name = "Movement OUT"
    verbose_name_plural = "Movements OUT"

    def get_queryset(self, request):
        qs = super().get_queryset(request)
        return qs.filter(direction=MovementDirection.OUT)


class SnapshotInline(admin.TabularInline):
    model = Snapshot
    extra = 1

    verbose_name = "Snapshot"
    verbose_name_plural = "Snapshots"


class AccountHolderRoleInline(admin.TabularInline):
    model = AccountHolderRole
    extra = 1
