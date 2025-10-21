#pragma once

#include "amount.h"
#include "ccy.h"

namespace finances::accounts::models {

    class Money {
      public:
        explicit Money(Amount amount, Ccy ccy);

        Money(Money&&) noexcept = default;
        Money& operator=(Money&&) noexcept = default;

        // Money(const Money&) = delete;
        // Money& operator=(const Money&) = delete;

        operator std::string() const;

      private:
        Amount amount;
        Ccy ccy;
    };

} // namespace finances::accounts::models
