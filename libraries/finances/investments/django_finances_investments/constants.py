from django_finances_accounts.constants import (
    AccountTypeConstants as AccountsAccountTypeConstants,
)
from django_finances_accounts.constants import (
    MovementTypeConstants as AccountsMovementTypeConstants,
)


class AccountTypeConstants(AccountsAccountTypeConstants):
    pass


class MovementTypeConstants(AccountsMovementTypeConstants):
    """Constant values associated to AccountType instances"""

    # AccountType.unique_name
    INVESTMENTS: str = "/income/investments"
    TAXES_USA_DIVIDEND_15: str = "/taxes/taxes/direct/usa/dividend15"
