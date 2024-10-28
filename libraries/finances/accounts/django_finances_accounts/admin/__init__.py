from django.contrib import admin
from django_finances_accounts.models import (
    Account,
    AccountHolder,
    AccountType,
    Custodian,
    Fx,
    Movement,
    MovementDividend,
    MovementNonNumerable,
    MovementNumerable,
    MovementType,
    SnapshotNonNumerable,
    SnapshotNumerable,
    Transaction,
    TransactionGroup,
)

from ._hierarchy_tree_model_admin import HierarchyTreeModelAdmin
from ._readonly_mixin import ReadOnlyAdminMixin
from ._render_change_form_mixin import RenderChangeFormMixin
from .inlines import (
    AccountHolderRoleInline,
    MovementDividendInline,
    MovementNonNumerableInline,
    MovementNumerableInline,
    SnapshotNonNumerableInline,
    SnapshotNumerableInline,
)

admin.site.register(SnapshotNonNumerable)
admin.site.register(SnapshotNumerable)


class AccountTypeModelAdmin(RenderChangeFormMixin, HierarchyTreeModelAdmin):
    change_form_help_text = (
        "<strong>Note.-</strong>. The hierarchy of account types is used to group several"
        " accounts into categories and create reports. Modifying this hierarchy or adding"
        " accounts outside it may have consequences on other applications"
    )
    list_filter = HierarchyTreeModelAdmin.list_filter + ("is_numerable",)
    list_display = HierarchyTreeModelAdmin.list_display + ("is_numerable",)


admin.site.register(AccountType, AccountTypeModelAdmin)


class MovementTypeModelAdmin(RenderChangeFormMixin, HierarchyTreeModelAdmin):
    change_form_help_text = (
        "<strong>Note.-</strong>. The hierarchy of movement types is"
        " used to group movements together and create reports."
        " Modifying this hierarchy or adding types outside the"
        " proposed one may have consequences in other applications."
    )


admin.site.register(MovementType, MovementTypeModelAdmin)


class TransactionGroupModelAdmin(RenderChangeFormMixin, admin.ModelAdmin):
    change_form_help_text = (
        "<strong>Note.-</strong>. Write only meaningful groups, do"
        " not overuse them. Ideally these groups will be used to create"
        " reports."
    )
    list_display = ("name", "cadence", "start")
    list_filter = ("cadence", "start")


admin.site.register(TransactionGroup, TransactionGroupModelAdmin)


class TransactionModelAdmin(admin.ModelAdmin):
    list_display = ("name", "group")
    list_filter = ("group",)
    inlines = [MovementNonNumerableInline, MovementNumerableInline, MovementDividendInline]


admin.site.register(Transaction, TransactionModelAdmin)


class AccountModelAdmin(admin.ModelAdmin):
    inlines = (
        AccountHolderRoleInline,
        SnapshotNumerableInline,
        SnapshotNonNumerableInline,
    )
    list_display = ("name", "custodian", "type", "open", "close")
    list_filter = ("custodian", "type__name", "close")

    def get_inline_instances(self, request, obj=None):
        # Return no inlines when obj is being created
        if not obj:
            return []
        unfiltered = super().get_inline_instances(request, obj)
        # filter out the Inlines you don't want
        if obj.type.is_numerable:
            filter = [AccountHolderRoleInline, SnapshotNumerableInline]
        else:
            filter = [AccountHolderRoleInline, SnapshotNonNumerableInline]
        return [x for x in unfiltered if any([isinstance(x, it) for it in filter])]


admin.site.register(Account, AccountModelAdmin)


class AccountHolderModelAdmin(admin.ModelAdmin):
    inlines = (AccountHolderRoleInline,)
    list_display = ("name", "is_company")


admin.site.register(AccountHolder, AccountHolderModelAdmin)


class CustodianModelAdmin(admin.ModelAdmin):
    list_display = (
        "name",
        "country",
    )


admin.site.register(Custodian, CustodianModelAdmin)


class FxModelAdmin(admin.ModelAdmin):
    list_display = (
        "local",
        "foreign",
        "date_value",
    )
    list_filter = ("date_value",)
    readonly_fields = ("local",)


admin.site.register(Fx, FxModelAdmin)


class MovementAdmin(admin.ModelAdmin):
    list_display = (
        "account",
        "date_value",
        "direction",
        "transaction__group",
        "type",
        "amount",
    )
    list_filter = ("account", "type", "date_value", "direction")
    search_fields = ("transaction__group",)


class ReadOnlyMovementAdmin(ReadOnlyAdminMixin, MovementAdmin):
    pass


admin.site.register(MovementNonNumerable, MovementAdmin)
admin.site.register(MovementNumerable, MovementAdmin)
admin.site.register(MovementDividend, MovementAdmin)
admin.site.register(Movement, ReadOnlyMovementAdmin)
