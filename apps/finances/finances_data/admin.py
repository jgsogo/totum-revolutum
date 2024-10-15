from django.contrib import admin
from treenode.admin import TreeNodeModelAdmin
from treenode.forms import TreeNodeForm

from .models import MovementType, AccountType

class AccountTypeAdmin(TreeNodeModelAdmin):
    list_display = ('is_abstract',)
    search_fields = ['name']
    list_filter = ('is_abstract', )

    treenode_display_mode = TreeNodeModelAdmin.TREENODE_DISPLAY_MODE_ACCORDION

    form = TreeNodeForm

admin.site.register(MovementType)
admin.site.register(AccountType, AccountTypeAdmin)
