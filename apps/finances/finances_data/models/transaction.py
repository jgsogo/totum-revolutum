from django.db import models

from ._hierarchy_tree import _HierarchyTree


class TransactionType(_HierarchyTree):
    pass


class Transaction(models.Model):
    """
    The reason why the associated group of movements took place.
    """

    name = models.CharField(max_length=255)
    description = models.TextField(null=True, blank=True)

    type = models.ForeignKey(TransactionType, on_delete=models.PROTECT)
