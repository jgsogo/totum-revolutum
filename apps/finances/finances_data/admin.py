from django.contrib import admin
from treenode.admin import TreeNodeModelAdmin
from treenode.forms import TreeNodeForm

from .models import (
    Account,
    AccountHolder,
    AccountType,
    Custodian,
    MovementType,
    SnapshotNonNumerable,
    SnapshotNumerable,
    Transaction,
    TransactionGroup,
    MovementNumerable,
    MovementNonNumerable,
    MovementDividend,
    Fx
)
from .models.account import AccountHolderRole


class RenderChangeFormMixin:
    def render_change_form(self, request, context, *args, **kwargs):
        self.change_form_template = "admin/finances_data/change_form.html"
        extra = {"help_text": self.change_form_help_text}

        context.update(extra)
        return super().render_change_form(request, context, *args, **kwargs)


class HierarchyTreeModelAdmin(TreeNodeModelAdmin):
    list_display = ("is_abstract",)
    search_fields = ["name"]
    list_filter = ("is_abstract",)

    treenode_display_mode = TreeNodeModelAdmin.TREENODE_DISPLAY_MODE_ACCORDION
    form = TreeNodeForm


class AccountTypeModelAdmin(RenderChangeFormMixin, HierarchyTreeModelAdmin):
    change_form_help_text = "<strong>Note.-</strong>. ."
    list_filter = HierarchyTreeModelAdmin.list_filter + ("is_numerable",)
    list_display = HierarchyTreeModelAdmin.list_display + ("is_numerable",)


class MovementTypeModelAdmin(RenderChangeFormMixin, HierarchyTreeModelAdmin):
    change_form_help_text = "<strong>Note.-</strong>. ."


class TransactionGroupModelAdmin(RenderChangeFormMixin, admin.ModelAdmin):
    change_form_help_text = "<strong>Note.-</strong>. ."
    list_display = ("name", "cadence", "start")
    list_filter = ("cadence", "start")

class MovementNonNumerableInline(admin.TabularInline):
    model = MovementNonNumerable
    extra = 1

class MovementNumerableInline(admin.TabularInline):
    model = MovementNumerable
    extra = 1

class MovementDividendInline(admin.TabularInline):
    model = MovementDividend
    extra = 1

class TransactionModelAdmin(RenderChangeFormMixin, admin.ModelAdmin):
    change_form_help_text = "<strong>Note.-</strong>. ."
    list_display = ("name", "group")
    list_filter = ("group",)
    inlines = [MovementNonNumerableInline, MovementNumerableInline, MovementDividendInline]


admin.site.register(MovementType, MovementTypeModelAdmin)
admin.site.register(AccountType, AccountTypeModelAdmin)
admin.site.register(TransactionGroup, TransactionGroupModelAdmin)

admin.site.register(Transaction, TransactionModelAdmin)
admin.site.register(SnapshotNonNumerable)
admin.site.register(SnapshotNumerable)


class SnapshotNonNumerableInline(admin.TabularInline):
    model = SnapshotNonNumerable
    extra = 1


class SnapshotNumerableInline(admin.TabularInline):
    model = SnapshotNumerable
    extra = 1


class AccountHolderRoleInline(admin.TabularInline):
    model = AccountHolderRole
    extra = 1


class AccountModelAdmin(RenderChangeFormMixin, admin.ModelAdmin):
    change_form_help_text = "<strong>Note.-</strong>. ."
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


class AccountHolderModelAdmin(RenderChangeFormMixin, admin.ModelAdmin):
    change_form_help_text = "<strong>Note.-</strong>. ."
    inlines = (AccountHolderRoleInline,)
    list_display = ("name", "is_company")


admin.site.register(AccountHolder, AccountHolderModelAdmin)


class CustodianModelAdmin(RenderChangeFormMixin, admin.ModelAdmin):
    change_form_help_text = "<strong>Note.-</strong>. ."
    list_display = (
        "name",
        "country",
    )


admin.site.register(Custodian, CustodianModelAdmin)


class FxModelAdmin(admin.ModelAdmin):
    list_display = ('local', 'foreign', 'date_value',)
    list_filter = ('date_value',)
    readonly_fields = ('local',)

admin.site.register(Fx, FxModelAdmin)


class MovementAdmin(admin.ModelAdmin):
    list_display = ('account', 'date_value', 'direction', 'transaction__group', 'type', 'get_amount')
    list_filter = ('account', 'type', 'date_value', 'direction')
    search_fields = ("transaction__group", )

admin.site.register(MovementNonNumerable, MovementAdmin)
admin.site.register(MovementNumerable, MovementAdmin)
admin.site.register(MovementDividend, MovementAdmin)