from django.core.validators import MinValueValidator
from django.db import models
from django.utils.translation import gettext_lazy as _

from .account import Account


class Snapshot(models.Model):
    account = models.ForeignKey(Account, on_delete=models.PROTECT)
    date_value = models.DateField()

    class Meta:
        abstract = True
        unique_together = [["account", "date_value"]]
        ordering = ["-date_value"]


class SnapshotNumerable(Snapshot):
    quantity = models.DecimalField(
        max_digits=14,
        decimal_places=4,
        validators=[MinValueValidator(0)],
        help_text=_("Typically this will be an integer, but some accounts allow fractional units"),
    )
    unit_value = models.DecimalField(
        max_digits=14, decimal_places=4, validators=[MinValueValidator(0)]
    )


class SnapshotNonNumerable(Snapshot):
    amount = models.DecimalField(max_digits=14, decimal_places=2, validators=[MinValueValidator(0)])
