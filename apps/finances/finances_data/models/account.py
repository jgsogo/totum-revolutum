from django.db import models
from django.utils.translation import gettext_lazy as _
from djmoney.models.fields import CurrencyField
from djmoney.settings import CURRENCY_CHOICES, DEFAULT_CURRENCY

from .account_holder import AccountHolder
from .account_type import AccountType
from .custodian import Custodian


class AccountHolderRole(models.Model):
    holder = models.ForeignKey(
        AccountHolder,
        on_delete=models.CASCADE,
        help_text=_(
            "Person designated and authorized to transact business on behalf of an account"
        ),
    )
    account = models.ForeignKey("Account", on_delete=models.CASCADE)

    owns_money = models.BooleanField(
        default=True,
        help_text=_(
            "This person will also be the owner of the money it contains. Furthermore,"
            " if there are any debt due to having taken out a loan, this person will be"
            " liable for the tax obligations"
        ),
    )


class Account(models.Model):
    name = models.CharField(max_length=255)
    description = models.TextField(null=True, blank=True)

    custodian = models.ForeignKey(
        Custodian,
        on_delete=models.PROTECT,
        help_text=_("The entity that has physical possession of its clients' financial assets"),
    )
    type = models.ForeignKey(AccountType, on_delete=models.PROTECT)
    identifier = models.CharField(
        max_length=255,
        help_text=_("Official account identifier"),
        null=True,
        blank=True,
    )

    holders = models.ManyToManyField(AccountHolder, through="AccountHolderRole")

    ccy = CurrencyField(default=DEFAULT_CURRENCY, choices=CURRENCY_CHOICES)

    open = models.DateField(
        help_text=_(
            "Date when this account was opened. All movements related to this"
            " account should happen after this data (and before close date)"
        )
    )
    close = models.DateField(
        blank=True,
        null=True,
        help_text=_(
            "Date when this account was closed. No more movements are allowed for this account"
        ),
    )

    def __str__(self) -> str:
        return f"{self.custodian} - {self.name}"
