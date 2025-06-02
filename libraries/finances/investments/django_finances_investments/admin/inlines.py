from django.contrib import admin
from django_finances_investments.models import (
    MovementNumerable,
    MovementDividend,
    SnapshotNumerable,
)
from django_finances_accounts.models import MovementDirection


class MovementNumerableInInline(admin.TabularInline):
    model = MovementNumerable
    extra = 1

    verbose_name = "MovementNumerable IN"
    verbose_name_plural = "MovementsNumerable IN"

    def get_queryset(self, request):
        qs = super().get_queryset(request)
        return qs.filter(direction=MovementDirection.IN)


class MovementNumerableOutInline(admin.TabularInline):
    model = MovementNumerable
    extra = 1

    verbose_name = "MovementNumerable OUT"
    verbose_name_plural = "MovementsNumerable OUT"

    def get_queryset(self, request):
        qs = super().get_queryset(request)
        return qs.filter(direction=MovementDirection.OUT)


class MovementDividendInInline(admin.TabularInline):
    model = MovementDividend
    extra = 1

    verbose_name = "MovementDividend IN"
    verbose_name_plural = "MovementsDividend IN"

    def get_queryset(self, request):
        qs = super().get_queryset(request)
        return qs.filter(direction=MovementDirection.IN)


class MovementDividendOutInline(admin.TabularInline):
    model = MovementDividend
    extra = 1

    verbose_name = "MovementDividend OUT"
    verbose_name_plural = "MovementsDividend OUT"

    def get_queryset(self, request):
        qs = super().get_queryset(request)
        return qs.filter(direction=MovementDirection.OUT)


class SnapshotNumerableInline(admin.TabularInline):
    model = SnapshotNumerable
    extra = 1
