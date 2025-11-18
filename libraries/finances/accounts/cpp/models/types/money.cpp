#include "money.h"

using namespace finances::accounts::models;

Money::Money(Amount amount, Ccy ccy) : amount{std::move(amount)}, ccy{std::move(ccy)} {}

Money::Money(Ccy ccy) : amount{}, ccy{std::move(ccy)} {}

Money::operator std::string() const {
    // We want to decimals for all (?) the currencies
    auto amount_str = dec::toString(decimal_cast<2>(amount.value), dec::decimal_format(','));
    return std::format("{} {}", amount_str, ccy);
}

Money& Money::operator+=(const Money& other) {
    if (ccy != other.ccy) {
        throw error::CcyMismatch{};
    }

    amount += other.amount;

    return *this;
}
