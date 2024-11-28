from django.core.exceptions import ValidationError
from django.db import models
from django.utils.translation import gettext_lazy as _
from django_finances_accounts.conf import settings

CADENCE_CHOICES = [
    ("once", "Once in a lifetime"),
    ("year", "Every year"),
    ("month", "Every month"),
    ("quarter", "Every quarter"),
]


def validate_movements(movs, money_tolerance: float):
    ins = []
    outs = []

    from .movement import Direction

    def collect_mov(mov):
        amount = mov.local_amount()
        if mov.direction == Direction.IN:
            ins.append(amount)
        else:
            outs.append(amount)

    for mov in movs:
        collect_mov(mov)

    ins = sum(ins)
    outs = sum(outs)

    if abs(ins - outs) > money_tolerance:
        raise ValidationError(
            f"INs ({ins}) has to be equal to OUTs ({outs}) with tolerance '{money_tolerance}'"
        )


class TransactionGroup(models.Model):
    """
    Groups several transactions and provides more context about them.

    Some examples:
     * Paying the rent for a given house
     * Fuel expenses related to one auto
     * Why we are giving away certain gift
     * All transactions related to some specific vacations

    Based on these groups, we can create reports. The 'start' date of
    a TransactionGroup should match the actual start date of the event,
    however some transactions associated to this group might happen
    before.
    """

    name = models.CharField(max_length=255)
    description = models.TextField(null=True, blank=True)

    cadence = models.CharField(
        max_length=10,
        choices=CADENCE_CHOICES,
        default="once",
        help_text=_("How often this same transaction group is rescheduled"),
    )
    start = models.DateField(
        help_text=_(
            "For recurring groups, this will be the base date to compute the cadence."
            " For non recurring events, this is just the date of the single occurrence"
        )
    )

    class Meta:
        ordering = ["name"]

    def __str__(self) -> str:
        return f"{self.name} ({self.cadence})"


class Transaction(models.Model):
    """
    The reason why the associated group of movements took place.
    """

    name = models.CharField(max_length=255)
    description = models.TextField(null=True, blank=True)

    group = models.ForeignKey(
        TransactionGroup,
        on_delete=models.PROTECT,
        null=True,
        blank=True,
        help_text=_("Which group (if any) this transaction belongs to"),
    )

    def clean(self):
        super().clean()

        # Validate movements (only if the transaction actually exists)
        if self.pk:
            all_movs = self.movement_set.all()
            validate_movements(all_movs, settings.FINANCES_MONEY_TOLERANCE)

    def __str__(self) -> str:
        if self.group:
            return f"{self.name} ({self.group.name})"
        else:
            return self.name
