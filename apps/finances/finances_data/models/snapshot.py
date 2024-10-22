from django.db import models
from django.utils.translation import gettext_lazy as _

from ._amount import AmountNonNumerableMixin, AmountNumerableMixin
from .account import Account


class Snapshot(models.Model):
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


class SnapshotNonNumerable(Snapshot, AmountNonNumerableMixin):
    """Snapshot for a non-numerable account"""

    pass


class SnapshotNumerable(Snapshot, AmountNumerableMixin):
    """Snapshot for a numerable account"""

    pass
