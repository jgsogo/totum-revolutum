#pragma once

#include "amount.h"
#include "ccy.h"

namespace finances::accounts::models {

    class Money {
      public:
        explicit Money(const Amount& amount, const Ccy& ccy);

        operator std::string() const;

      private:
        const Amount& amount;
        const Ccy& ccy;
    };

} // namespace finances::accounts::models
