from django.db import models
from django.core.validators import MinValueValidator
from treenode.models import TreeNodeModel

class _HierarchyTree(TreeNodeModel):
    # parent = models.ForeignKey('self', null=True, blank=True, on_delete=models.SET_NULL)
    name = models.CharField(max_length=255)
    description = models.TextField(null=True, blank=True)
    is_abstract = models.BooleanField(default=False, help_text="If the category can be instantiated or not")
    # level = models.PositiveSmallIntegerField(editable=False, validators=[MinValueValidator(0)])

    treenode_display_field = "name"

    class Meta(TreeNodeModel.Meta):
        abstract = True
        unique_together = [["tn_parent", "name"]]

    def __str__(self):
        if self.parent:
            return f"{self.parent} :: {self.name}"
        else:
            return f"{self.name}"

    # def clean(self):
    #     super().clean()
    #     if self.parent:
    #         self.level = self.parent.level + 1
    #     else:
    #         self.level = 0
