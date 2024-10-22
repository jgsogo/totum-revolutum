from django.core.exceptions import ObjectDoesNotExist, ValidationError
from django.core.validators import MinValueValidator
from django.db import models
from django.utils.translation import gettext_lazy as _
from djmoney.settings import BASE_CURRENCY

from ._amount import AmountNonNumerableMixin, AmountNumerableMixin
from .account import Account
from .fx import Fx
from .movement_type import MovementType
from .transaction import Transaction


class Direction(models.IntegerChoices):
    IN = 0, _("IN")
    OUT = 1, _("OUT")


class Movement(models.Model):
    transaction = models.ForeignKey(
        Transaction,
        on_delete=models.CASCADE,
        help_text=_(
            "Every movement is associated to a transaction,"
            " the transaction contains its counterparts"
        ),
    )
    type = models.ForeignKey(MovementType, on_delete=models.PROTECT)
    direction = models.IntegerField(
        choices=Direction.choices,
        help_text=_(
            "Whether the movement substracts an amount from the related"
            " account (OUT) or increases it (IN)"
        ),
    )

    account = models.ForeignKey(Account, on_delete=models.PROTECT)
    date_value = models.DateField(
        help_text=_("Date when the movement is annotated in the associated account")
    )

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

    def clean(self):
        super().clean()
        if self.fx:
            if self.date_value != self.fx.date_value:
                raise ValidationError("Date_value for FX and Movement didn't match")
            if self.fx.foreign != self.account.ccy:
                raise ValidationError("FX foreign currency and Account currency didn't match")
        if self.date_value < self.account.open:
            raise ValidationError("Movement date_value should be after Account open")
        if self.account.close and self.date_value > self.account.close:
            raise ValidationError("Movement date_vale cannot be after account is closed")

        if self.account.ccy != BASE_CURRENCY:
            if not self.fx:
                raise ValidationError("FX is required if the account is not in base currency")

    def local_amount(self) -> float:
        amount = self.get_amount()
        if self.fx:
            return amount * self.fx.inverse()
        else:
            return amount


class MovementNonNumerable(Movement, AmountNonNumerableMixin):
    """A movement that only involves an amount"""

    pass


class MovementNumerable(Movement, AmountNumerableMixin):
    """A movement that modifies the number of units in a numerable account (buy/sell units)"""

    pass


class MovementDividend(Movement):
    """(Only for 'Stock' accounts) Dividend paid by an account"""

    ex_dividend_date = models.DateField(
        help_text=_(
            "Date when the dividend is assigned. This data is used to"
            " retrieved the number of stocks"
        )
    )

    unit_value = models.DecimalField(
        max_digits=14,
        decimal_places=4,
        validators=[MinValueValidator(0, "Unit value should be equal or greater than 0")],
        help_text=_("Dividen per stock"),
    )

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

    def get_amount(self):
        snapshot = self.account.snapshotnumerable_set.filter(
            date_value__lte=self.ex_dividend_date
        ).first()
        if not snapshot:
            raise ObjectDoesNotExist(
                "Associated account doesn't have any Snapshot previous to the dividend ex_date"
            )

        return self.unit_value * float(snapshot.quantity)
