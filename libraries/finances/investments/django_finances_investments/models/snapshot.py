from django_finances_accounts.models.snapshot import BaseSnapshot

from ._amount_numerable import AmountNumerableMixin


class SnapshotNumerable(BaseSnapshot, AmountNumerableMixin):
    """Snapshot for a numerable account"""

    pass
