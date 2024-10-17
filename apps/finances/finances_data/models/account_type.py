from django.db import models
from django.utils.translation import gettext_lazy as _

from ._hierarchy_tree import _HierarchyTree


class AccountType(_HierarchyTree):
    is_numerable = models.BooleanField(
        default=False,
        help_text=_(
            "Account with stocks are numerable (units), a bank account or a mortage is not"
        ),
    )
