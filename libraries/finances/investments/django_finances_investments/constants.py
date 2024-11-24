from django_finances_accounts.constants import (
    AccountTypeConstants as AccountsAccountTypeConstants,
)
from django_finances_accounts.constants import (
    MovementTypeConstants as AccountsMovementTypeConstants,
)


class AccountTypeConstants(AccountsAccountTypeConstants):
    ASSETS_CURRENT_INVESTMENT: str = "/assets/current/investment"
    ASSETS_NON_CURRENT_REALSTATE: str = "/assets/non-current/real-state"
    ASSETS_NON_CURRENT_RETIREMENTPLAN: str = "/assets/non-current/retirement"


class MovementTypeConstants(AccountsMovementTypeConstants):
    """Constant values associated to AccountType instances"""

    # AccountType.unique_name
    INVESTMENTS: str = "/income/investments"
    TAXES_USA_DIVIDEND_15: str = "/taxes/taxes/direct/usa/dividend15"
