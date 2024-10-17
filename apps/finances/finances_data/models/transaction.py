from django.db import models
from django.utils.translation import gettext_lazy as _

CADENCE_CHOICES = [
    ("once", "Once in a lifetime"),
    ("year", "Every year"),
    ("month", "Every month"),
    ("quarter", "Every quarter"),
]


class TransactionGroup(models.Model):
    """
    Groups several transactions and provides more context about them.

    Some examples:
     * Paying the rent for a given house
     * Fuel expenses related to one auto
     * Why we are giving away certain gift
     * All transactions related to some specific vacations

    Based on these groups, we can create reports
    """

    name = models.CharField(max_length=255)
    description = models.TextField(null=True, blank=True)

    cadence = models.CharField(
        max_length=10,
        choices=CADENCE_CHOICES,
        default="once",
        help_text=_("How often this same group is rescheduled"),
    )
    start = models.DateField(
        help_text=_(
            "For recurring groups, this will be the base date to compute the cadence."
            " For non recurring events, this is just the date of the single occurrence"
        )
    )

    def __str__(self) -> str:
        return self.name


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

    def __str__(self) -> str:
        return self.name
