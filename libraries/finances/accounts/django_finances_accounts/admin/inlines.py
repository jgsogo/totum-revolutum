from django.contrib import admin
from django_finances_accounts.models import Movement, Snapshot
from django_finances_accounts.models.account import AccountHolderRole


class MovementInline(admin.TabularInline):
    model = Movement
    extra = 1


class SnapshotInline(admin.TabularInline):
    model = Snapshot
    extra = 1


class AccountHolderRoleInline(admin.TabularInline):
    model = AccountHolderRole
    extra = 1
