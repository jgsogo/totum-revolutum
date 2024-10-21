from django.core.validators import MinValueValidator
from django.db import models
from django.utils.translation import gettext_lazy as _

from .account import Account
from .fx import Fx
from .movement_type import MovementType
from .transaction import Transaction


class Direction(models.IntegerChoices):
    IN = 0, _("IN")
    OUT = 1, _("OUT")


class Movement(models.Model):
    transaction = models.ForeignKey(Transaction, on_delete=models.CASCADE)
    type = models.ForeignKey(MovementType, on_delete=models.PROTECT)
    direction = models.IntegerField(choices=Direction.choices)

    account = models.ForeignKey(Account, on_delete=models.PROTECT)
    date_value = models.DateField()

    fx = models.ForeignKey(
        Fx,
        blank=True,
        null=True,
        on_delete=models.PROTECT,
        help_text=_(
            "This is the FX used for this movement (it doesn't need to be exactly the same"
            " as the official one for this 'date_value', as FX varies along the day)"
        ),
    )

    class Meta:
        abstract = True
        ordering = ["-date_value"]

    def get_amount(self):
        raise NotImplementedError


class MovementNonNumerable(Movement):
    """A movement that only involves an amount"""

    amount = models.DecimalField(max_digits=14, decimal_places=2, validators=[MinValueValidator(0)])

    def get_amount(self):
        return self.amount

class MovementNumerable(Movement):
    """A movement that modifies the number of units in a numerable account (buy/sell units)"""

    quantity = models.DecimalField(
        max_digits=14,
        decimal_places=4,
        validators=[MinValueValidator(0)],
        help_text=_(
            "Number of units. Typically this will be an integer, but some accounts allow"
            " fractional units"
        ),
    )
    unit_value = models.DecimalField(
        max_digits=14,
        decimal_places=4,
        validators=[MinValueValidator(0)],
        help_text=_("Value per unit"),
    )

    def get_amount(self):
        return self.quantity * self.unit_value

class MovementDividend(Movement):
    """(Only for 'Stock' accounts) Dividend paid by an account"""

    ex_dividend_date = models.DateField(help_text=_("Date when the dividend is assigned"))

    unit_value = models.DecimalField(
        max_digits=14,
        decimal_places=4,
        validators=[MinValueValidator(0)],
        help_text=_("Dividen per stock"),
    )

    def get_amount(self):
        snapshot = self.account.snapshot_numerable_set.filter(date_value__lte=self.ex_dividend_date).first()
        return self.unit_value * snapshot.quantity
    