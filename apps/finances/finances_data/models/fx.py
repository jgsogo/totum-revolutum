from django.core.exceptions import ValidationError
from django.core.validators import MinValueValidator
from django.db import models
from django.utils.translation import gettext_lazy as _
from djmoney.models.fields import CurrencyField
from djmoney.settings import BASE_CURRENCY, CURRENCY_CHOICES


class Fx(models.Model):
    foreign = CurrencyField(choices=CURRENCY_CHOICES, help_text=_("Foreign (quoted) currency"))
    local = CurrencyField(
        choices=CURRENCY_CHOICES, default=BASE_CURRENCY, help_text=_("Local/foreign currency")
    )

    rate = models.DecimalField(
        max_digits=14,
        decimal_places=4,
        validators=[MinValueValidator(0)],
        help_text=_("How many 'foreign' you need to take a 'local'"),
    )
    date_value = models.DateField()

    class Meta:
        ordering = ["-date_value"]

    def __str__(self):
        return f"{self.local}/{self.foreign} = {self.rate} ({self.date_value})"

    def clean(self):
        if self.foreign == self.local:
            raise ValidationError(_("Equal local and foreign currencies makes no sense"))
