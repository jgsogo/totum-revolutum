from decimal import Decimal
from typing import Optional

from django_finances_accounts.models.movement import Direction


def ccy2str(
    value: Decimal,
    ccy: str,
    decimal_places: int = 2,
    max_digits: Optional[int] = None,
    direction: Optional[Direction] = None,
):
    format_str = f"{{:.{decimal_places}f}}"
    the_number = format_str.format(abs(value))

    if max_digits and max_digits > len(the_number):
        the_number = "{}{}".format(" " * (max_digits - len(the_number)), the_number)

    symbol_str = ""
    if direction is not None:
        symbol_str = "+ " if direction == Direction.IN else "- "
        symbol_str = symbol_str if value != 0.0 else "  "

    return f"{symbol_str}{the_number} {ccy}"
