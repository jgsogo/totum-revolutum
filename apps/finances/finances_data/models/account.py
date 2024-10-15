from django.db import models
from .custodian import Custodian
from .account_type import AccountType
from .account_holder import AccountHolder
from django.utils.translation import gettext_lazy as _
from djmoney.models.fields import CurrencyField

from djmoney.settings import CURRENCY_CHOICES, CURRENCY_CODE_MAX_LENGTH, DECIMAL_PLACES, DEFAULT_CURRENCY

class AccountHolderRole(models.Model):
    holder = models.ForeignKey(AccountHolder, on_delete=models.CASCADE)
    account = models.ForeignKey('Account', on_delete=models.CASCADE)

    owns_money = models.BooleanField(default=True, help_text=_("This person will also be the owner of the money it contains. Furthermore, if there are any debt due to having taken out a loan, this person will be liable for the tax obligations"))


class Account(models.Model):
    name = models.CharField(max_length=255)
    description = models.TextField(null=True, blank=True)

    custodian = models.ForeignKey(Custodian, on_delete=models.PROTECT)
    type = models.ForeignKey(AccountType, on_delete=models.PROTECT)
    identifier = models.CharField(max_length=255, help_text=_("Official account identifier"), null=True, blank=True)

    holders = models.ManyToManyField(AccountHolder, through='AccountHolderRole')

    # FIXME, Maybe the AccountType already contains this information?
    is_numerable = models.BooleanField(default=False,
                                       help_text=_("Account with stocks is numerable (units), a bank account isn't"))
    ccy = CurrencyField(default=DEFAULT_CURRENCY, choices=CURRENCY_CHOICES)


    open = models.DateField()
    close = models.DateField(blank=True, null=True)
