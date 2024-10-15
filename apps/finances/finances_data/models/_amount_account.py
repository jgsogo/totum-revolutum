from data.constants import DECIMAL_PLACES
from django.core.validators import MinValueValidator
from django.db import models


class AmountBase(models.Model):
    """Represents something monetizable. It can be cash or a number of stocks"""

    amount = models.DecimalField(max_digits=14, decimal_places=DECIMAL_PLACES, blank=True)

    quantity = models.PositiveIntegerField(
        null=True, blank=True, validators=[MinValueValidator(0)]
    )  # Need the validator, sqlite3 disables it
    unit_value = models.DecimalField(
        max_digits=14, decimal_places=DECIMAL_PLACES, null=True, blank=True
    )

    class Meta:
        abstract = True
