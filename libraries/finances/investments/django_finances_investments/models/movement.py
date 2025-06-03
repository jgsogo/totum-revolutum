from django.core.exceptions import ObjectDoesNotExist, ValidationError
from django.core.validators import MinValueValidator
from django.db import models
from django.utils.translation import gettext_lazy as _
from django_finances_accounts.models import Movement

from ._amount_numerable import AmountNumerableMixin
from .snapshot import SnapshotNumerable


class MovementNumerable(AmountNumerableMixin, Movement):
    """A movement that modifies the number of units in a numerable account (buy/sell units)"""

    class Meta:
        ordering = ["-date_value"]

    def save(self, *args, **kwargs):
        self.amount = self.get_amount()
        super().save(*args, **kwargs)


class MovementDividend(Movement):
    """Dividend paid by an account"""

    ex_dividend_date = models.DateField(
        help_text=_(
            "Date when the dividend is assigned. This data is used to"
            " retrieve the number of stocks"
        )
    )

    unit_value = models.DecimalField(
        max_digits=14,
        decimal_places=4,
        validators=[MinValueValidator(0, "Unit value should be equal or greater than 0")],
        help_text=_("Dividen per stock"),
    )

    class Meta:
        ordering = ["-date_value"]

    def save(self, *args, **kwargs):
        self.amount = self.get_amount()
        super().save(*args, **kwargs)

    def clean(self):
        super().clean()
        if self.ex_dividend_date < self.account.open:
            raise ValidationError("MovementDividend ex_dividend_date should be after Account open")
        if self.account.close and self.ex_dividend_date > self.account.close:
            raise ValidationError(
                "MovementDividend ex_dividend_date cannot be after account is closed"
            )
        if self.ex_dividend_date > self.date_value:
            raise ValidationError("MovementDividend ex_dividend_date should be before date_value")

    def _snapshot(self):
        snapshot = SnapshotNumerable.objects.filter(
            snapshot_ptr__account=self.account, snapshot_ptr__date_value__lte=self.ex_dividend_date
        ).first()
        if not snapshot:
            raise ObjectDoesNotExist(
                "Associated account doesn't have any Snapshot previous to the dividend ex_date."
                " Create the snapshot first"
            )
        return snapshot

    def get_amount(self):
        snapshot = self._snapshot()
        return self.unit_value * float(snapshot.quantity)
