from django.core.exceptions import ValidationError
from django.db import models
from django.utils.translation import gettext_lazy as _

from ._amount import AmountMixin
from .account import Account


class BaseSnapshot(models.Model):
    account = models.ForeignKey(
        Account, on_delete=models.PROTECT, help_text=_("Account this snapshot refers to")
    )
    date_value = models.DateField(
        help_text=_(
            "Date corresponding to this snapshot. It should be a working day,"
            " so the FX can be retrieved from official sources"
        )
    )

    class Meta:
        abstract = True
        unique_together = [["account", "date_value"]]
        ordering = ["-date_value"]

    def __str__(self):
        return f"{self.account} @ {self.date_value}"

    def clean(self):
        super().clean()
        if self.date_value < self.account.open:
            raise ValidationError("Snapshot date_vale cannot be before account is opened")
        if self.account.close and self.date_value > self.account.close:
            raise ValidationError("Snapshot date_vale cannot be after account is closed")


class Snapshot(BaseSnapshot, AmountMixin):
    """Snapshot for an account"""

    pass
