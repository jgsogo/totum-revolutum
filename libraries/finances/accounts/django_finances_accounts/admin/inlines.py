from django.contrib import admin
from django_finances_accounts.models import MovementNonNumerable, SnapshotNonNumerable
from django_finances_accounts.models.account import AccountHolderRole


class MovementNonNumerableInline(admin.TabularInline):
    model = MovementNonNumerable
    extra = 1


class SnapshotNonNumerableInline(admin.TabularInline):
    model = SnapshotNonNumerable
    extra = 1


class AccountHolderRoleInline(admin.TabularInline):
    model = AccountHolderRole
    extra = 1
