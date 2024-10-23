from treenode.admin import TreeNodeModelAdmin
from treenode.forms import TreeNodeForm


class HierarchyTreeModelAdmin(TreeNodeModelAdmin):
    """A base ModelAdmin class for the models that inherit from django-treenode models"""

    list_display = ("is_abstract",)
    search_fields = ["name"]
    list_filter = ("is_abstract",)

    treenode_display_mode = TreeNodeModelAdmin.TREENODE_DISPLAY_MODE_ACCORDION
    form = TreeNodeForm
