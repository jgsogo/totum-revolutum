#include "fx.h"

#include <spdlog/spdlog.h>

#include "errors.h"

namespace finances::accounts::models {

    Money apply_fx(const Money& money, const Fx& fx) {
        SPDLOG_DEBUG("apply_fx(money='{}', fx='{}')", static_cast<std::string>(money), static_cast<std::string>(fx));

        if (!fx.is_valid()) {
            throw error::FXRateInvalid{static_cast<std::string>(fx)};
        }

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

    Fx::operator std::string() const {
        auto rate_str = dec::toString(dec::decimal_cast<4>(rate.value), dec::decimal_format(','));
        return std::format("{} {}/{}", rate_str, local, foreign);
    }

    bool Fx::is_valid() const { return rate.value != 0; }

} // namespace finances::accounts::models
