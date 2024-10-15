from django.contrib import admin
from treenode.admin import TreeNodeModelAdmin
from treenode.forms import TreeNodeForm

from .models import MovementType, AccountType, TransferType, AccountHolder, Account, Custodian, Transfer, SnapshotNonNumerable, SnapshotNumerable
from .models.account import AccountHolderRole

class RenderChangeFormMixin:
    def render_change_form(self, request, context, *args, **kwargs):
        self.change_form_template = 'admin/finances_data/change_form.html'
        extra = {
            'help_text': self.change_form_help_text
        }

        context.update(extra)
        return super().render_change_form(request, context, *args, **kwargs)

class HierarchyTreeModelAdmin(TreeNodeModelAdmin):
    list_display = ('is_abstract',)
    search_fields = ['name']
    list_filter = ('is_abstract', )

    treenode_display_mode = TreeNodeModelAdmin.TREENODE_DISPLAY_MODE_ACCORDION
    form = TreeNodeForm


class AccountTypeModelAdmin(RenderChangeFormMixin, HierarchyTreeModelAdmin):
    change_form_help_text = "<strong>Note.-</strong>. ."


class MovementTypeModelAdmin(RenderChangeFormMixin, HierarchyTreeModelAdmin):
    change_form_help_text = "<strong>Note.-</strong>. ."

class TransferTypeModelAdmin(RenderChangeFormMixin, HierarchyTreeModelAdmin):
    change_form_help_text = "<strong>Note.-</strong>. ."



admin.site.register(MovementType, MovementTypeModelAdmin)
admin.site.register(AccountType, AccountTypeModelAdmin)
admin.site.register(TransferType, TransferTypeModelAdmin)
admin.site.register(Custodian)
admin.site.register(Transfer)
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
    inlines = (AccountHolderRoleInline, )

    def get_inline_instances(self, request, obj=None):
        #Return no inlines when obj is being created
        if not obj:
            return []
        unfiltered = super().get_inline_instances(request, obj)
        #filter out the Inlines you don't want
        if obj.is_numerable:
            filter = [AccountHolderRoleInline, SnapshotNumerableInline]
        else:
            filter = [AccountHolderRoleInline, SnapshotNonNumerableInline]
        return [x for x in unfiltered if any(isinstance(x, it) for it in filter)]


admin.site.register(Account, AccountModelAdmin)

class AccountHolderRoleInline(admin.TabularInline):
    model = AccountHolderRole
    extra = 1

class AccountHolderModelAdmin(RenderChangeFormMixin, admin.ModelAdmin):
    change_form_help_text = "<strong>Note.-</strong>. ."
    inlines = (AccountHolderRoleInline,)

admin.site.register(AccountHolder, AccountHolderModelAdmin)
