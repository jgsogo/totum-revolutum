from django.db import models
from django.utils.translation import gettext_lazy as _
from django_countries.fields import CountryField


class Custodian(models.Model):
    """
    A bank custodian has physical possession of its clients' financial assets. These could
    include cash, stock certificates, bonds, and other financial instruments.
    """

    name = models.CharField(max_length=255, unique=True)
    description = models.TextField(null=True, blank=True)

    country = CountryField(help_text=_("Country where this custodian runs its business"))

    def __str__(self) -> str:
        return self.name
