from django.db import models
from django.utils.translation import gettext_lazy as _
from treenode.models import TreeNodeModel


class _HierarchyTree(TreeNodeModel):
    name = models.CharField(max_length=255, help_text=_("Name of the element"))
    description = models.TextField(
        null=True, blank=True, help_text=_("A description of the element")
    )
    is_abstract = models.BooleanField(
        default=False, help_text="If the category can be instantiated or not"
    )

    treenode_display_field = "name"

    class Meta(TreeNodeModel.Meta):
        abstract = True
        unique_together = [["tn_parent", "name"]]

    def __str__(self):
        try:
            # This 'it.name' can fail while populating the fixtures because the parent
            # might not have been added yet to the database
            return " / ".join([it.name for it in self.get_breadcrumbs(attr=None)])
        except:  # noqa: E722
            return self.name
