from django.db import models

from ._hierarchy_tree import _HierarchyTree


class TransferType(_HierarchyTree):
    pass


class Transfer(models.Model):
    """
    The reason why the associated group of movements took place.
    """

    name = models.CharField(max_length=255)
    description = models.TextField(null=True, blank=True)

    type = models.ForeignKey(TransferType, on_delete=models.PROTECT)
