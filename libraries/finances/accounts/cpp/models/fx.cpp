#include "fx.h"

#include <spdlog/spdlog.h>

#include "errors.h"
#include "model_manager.hpp"

using namespace finances::accounts::models;

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
} // namespace finances::accounts::models

Fx::operator std::string() const {
    auto rate_str = dec::toString(dec::decimal_cast<4>(rate.value), dec::decimal_format(','));
    return std::format("{} {}/{}", rate_str, local, foreign);
}

bool Fx::is_valid() const { return rate.value != 0; }

namespace utils::db {

    template <> Id ModelManager<Fx>::_create(pqxx::work& tx, const Fx& fx) {
        SPDLOG_DEBUG("Create a new FX");

        auto query = std::format("INSERT INTO {}"
                                 " (foreign, local, rate, date_value)"
                                 " VALUES ($1, $2, $3, $4)"
                                 " RETURNING id;",
                                 FX_TABLE);

        SPDLOG_TRACE(query);
        auto r = tx.exec(query, pqxx::params{static_cast<std::string_view>(fx.foreign),
                                             static_cast<std::string_view>(fx.local), fx.rate, fx.date_value})
                     .one_field();
        auto inserted_id = r.as<Id>();
        return inserted_id;
    }

} // namespace utils::db
