#include "fx.h"

#include "errors.h"

namespace finances::accounts::models {

    Money apply_fx(const Money& money, const Fx& fx) {
        if (money.ccy == fx.foreign) {
            Amount new_amount = money.amount / fx.rate;
            return Money{std::move(new_amount), fx.local};
        } else if (money.ccy == fx.local) {
            Amount new_amount = money.amount * fx.rate;
            return Money{std::move(new_amount), fx.foreign};
        } else {
            throw error::CcyMismatch{};
        }
    }
} // namespace finances::accounts::models
