from django.contrib import admin
from django_finances_accounts.models import MovementDirection
from django_finances_investments.models import (
    MovementDividend,
    MovementNumerable,
)


class MovementNumerableInInline(admin.TabularInline):
    model = MovementNumerable
    readonly_fields = ("amount",)
    extra = 1

    verbose_name = "Movement Numerable IN"
    verbose_name_plural = "Movements Numerable IN"

    def get_queryset(self, request):
        qs = super().get_queryset(request)
        return qs.filter(direction=MovementDirection.IN)


class MovementNumerableOutInline(admin.TabularInline):
    model = MovementNumerable
    readonly_fields = ("amount",)
    extra = 1

    verbose_name = "Movement Numerable OUT"
    verbose_name_plural = "Movements Numerable OUT"

    def get_queryset(self, request):
        qs = super().get_queryset(request)
        return qs.filter(direction=MovementDirection.OUT)


class MovementDividendInInline(admin.TabularInline):
    model = MovementDividend
    readonly_fields = ("amount",)
    extra = 1

    verbose_name = "Movement Dividend IN"
    verbose_name_plural = "Movements Dividend IN"

    def get_queryset(self, request):
        qs = super().get_queryset(request)
        return qs.filter(direction=MovementDirection.IN)


class MovementDividendOutInline(admin.TabularInline):
    model = MovementDividend
    readonly_fields = ("amount",)
    extra = 1

    verbose_name = "Movement Dividend OUT"
    verbose_name_plural = "Movements Dividend OUT"

    def get_queryset(self, request):
        qs = super().get_queryset(request)
        return qs.filter(direction=MovementDirection.OUT)
