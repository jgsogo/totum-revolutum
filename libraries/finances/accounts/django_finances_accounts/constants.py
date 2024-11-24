class AccountTypeConstants:
    """Constant values associated to AccountType instances"""

    # AccountType.unique_name
    ASSETS: str = "/assets"
    LIABILITIES: str = "/liabilities"
    ASSETS_CURRENT: str = "/assets/current"
    ASSETS_NONCURRENT: str = "/assets/non-current"
    ASSETS_CURRENT_SAVINGS: str = "/assets/current/savings"


class MovementTypeConstants:
    """Constant values associated to MovementType instances"""

    # MovementType.unique_name
    EXPENSE: str = "/expense"
    INCOME: str = "/income"
    OPERATIONS: str = "/operations"
    TAXES: str = "/taxes"

    ESPECIAL_TAXES: str = "/taxes/special-contributions"
    INCOME_AND_VALUE_TAXES: str = "/taxes/income-and-value"
    FEES_AND_TOLLS: str = "/taxes/fees-and-tolls"

    DIRECT_TAXES: str = "/taxes/income-and-value/direct"
    INDIRECT_TAXES: str = "/taxes/income-and-value/indirect"
