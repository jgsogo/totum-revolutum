from django import forms
from django.contrib import admin
from django_finances_accounts.models import Movement, MovementDirection, Snapshot
from django_finances_accounts.models.account import AccountHolderRole


def get_movement_inline_form(direction: MovementDirection) -> forms.ModelForm:
    """Returns a `ModelForm` (for `Movement` model)

    The form returned will hardcode the `Movement.direction` field to the
    value provide in the input argument
    """

    class MovementForm(forms.ModelForm):
        def clean(self):
            self.instance.direction = direction
            return super().clean()

    return MovementForm


class MovementInInline(admin.TabularInline):
    model = Movement
    extra = 1
    form = get_movement_inline_form(MovementDirection.IN)
    exclude = ("direction",)

    verbose_name = "Movement IN"
    verbose_name_plural = "Movements IN"

    def get_queryset(self, request):
        qs = super().get_queryset(request)
        return qs.filter(direction=MovementDirection.IN)


class MovementOutInline(admin.TabularInline):
    model = Movement
    extra = 1
    form = get_movement_inline_form(MovementDirection.OUT)
    exclude = ("direction",)

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
