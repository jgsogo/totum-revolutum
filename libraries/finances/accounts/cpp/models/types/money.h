#pragma once

#include "amount.h"
#include "ccy.h"

namespace finances::accounts::models {

    class Money {
      public:
        explicit Money(Amount amount, Ccy ccy);

        Money(Money&&) noexcept = default;
        Money& operator=(Money&&) noexcept = default;

        operator std::string() const;

        friend Money operator*(const Money& lhs, const Amount& rhs);

      private:
        Amount amount;
        Ccy ccy;
    };

    inline Money operator*(const Money& lhs, const Amount& rhs) {
        auto new_amount = lhs.amount * rhs;
        return Money{std::move(new_amount), lhs.ccy};
    }

} // namespace finances::accounts::models
