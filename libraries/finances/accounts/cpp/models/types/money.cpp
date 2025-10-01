#include "money.h"

using namespace finances::accounts::models;

Money::Money(const Amount& amount, const Ccy& ccy) : amount{amount}, ccy{ccy} {}

Money::operator std::string() const {
    // We want to decimals for all (?) the currencies
    auto amount_str = dec::toString(decimal_cast<2>(amount.value), dec::decimal_format(','));
    return std::format("{} {}", amount_str, ccy);
}
