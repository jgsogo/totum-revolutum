from django.db import models
from django.utils.translation import gettext_lazy as _


class AccountHolder(models.Model):
    """
    Any and all persons designated and authorized to transact business on behalf
    of an account. Each account holder's signature needs to be on file with the bank.
    The signature authorizes that person to conduct business on behalf of the account.
    """
    name = models.CharField(max_length=255, unique=True)
