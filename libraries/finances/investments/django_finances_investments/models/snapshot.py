from django_finances_accounts.models.snapshot import Snapshot

from ._amount_numerable import AmountNumerableMixin


class SnapshotNumerable(AmountNumerableMixin, Snapshot):
    """Snapshot for a numerable account"""

    class Meta:
        ordering = ["-date_value"]

    def save(self, *args, **kwargs):
        self.amount = self.get_amount()
        super().save(*args, **kwargs)
