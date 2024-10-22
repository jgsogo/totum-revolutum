from django.contrib import admin
from finances_data.models import (
    MovementDividend,
    MovementNonNumerable,
    MovementNumerable,
    SnapshotNonNumerable,
    SnapshotNumerable,
)
from finances_data.models.account import AccountHolderRole


class MovementNonNumerableInline(admin.TabularInline):
    model = MovementNonNumerable
    extra = 1


class MovementNumerableInline(admin.TabularInline):
    model = MovementNumerable
    extra = 1


class MovementDividendInline(admin.TabularInline):
    model = MovementDividend
    extra = 1


class SnapshotNonNumerableInline(admin.TabularInline):
    model = SnapshotNonNumerable
    extra = 1


class SnapshotNumerableInline(admin.TabularInline):
    model = SnapshotNumerable
    extra = 1


class AccountHolderRoleInline(admin.TabularInline):
    model = AccountHolderRole
    extra = 1
