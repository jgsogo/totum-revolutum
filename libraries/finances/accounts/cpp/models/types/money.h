#pragma once

#include "amount.h"
#include "ccy.h"

#include "libraries/finances/accounts/cpp/models/errors.h"

namespace finances::accounts::models {

    class Money {
      public:
        explicit Money(Amount amount, Ccy ccy);
        explicit Money(Ccy ccy);

        Money(Money&&) noexcept = default;
        Money(const Money&) noexcept = default;

        Money& operator=(Money&&) noexcept = default;

        operator std::string() const;

        Money& operator+=(const Money& other);

        friend Money operator*(const Money& lhs, const Amount& rhs);
        friend Money operator+(const Money& lhs, const Money& rhs);
        friend bool operator==(const Money& lhs, const Money& rhs);

      public:
        Amount amount;
        Ccy ccy;
    };

    inline Money operator*(const Money& lhs, const Amount& rhs) {
        auto new_amount = lhs.amount * rhs;
        return Money{std::move(new_amount), lhs.ccy};
    }

    inline Money operator+(const Money& lhs, const Money& rhs) {
        if (lhs.ccy != rhs.ccy) {
            throw error::CcyMismatch{};
        }

        return Money{lhs.amount + rhs.amount, lhs.ccy};
    }

    inline bool operator==(const Money& lhs, const Money& rhs) {
        return (lhs.ccy == rhs.ccy) && (lhs.amount == rhs.amount);
    }

} // namespace finances::accounts::models
