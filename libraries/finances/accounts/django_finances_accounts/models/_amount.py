from django.core.validators import MinValueValidator
from django.db import models
from django.utils.translation import gettext_lazy as _


class AmountNonNumerableMixin(models.Model):
    """A mixin for a model storing an 'amount' (non numerable amount)"""

    amount = models.DecimalField(
        max_digits=14,
        decimal_places=2,
        validators=[MinValueValidator(0, "Amount should be equal or greater than 0")],
        help_text=_("Amount in object's currency"),
    )

    class Meta:
        abstract = True

    def get_amount(self):
        return self.amount
