#pragma once

#include "libraries/finances/accounts/cpp/models/movement.h"

namespace finances::investments::models {
    class MovementDividendManager;

    struct MovementDividend {
        using Manager = MovementDividendManager;

        finances::accounts::models::Movement movement;

        finances::accounts::models::Id id;
        utils::libpqxx::Date ex_dividend_date;
        finances::accounts::models::Amount unit_value;

        std::optional<std::pair<utils::libpqxx::Date, finances::accounts::models::Amount>>
            snapshot_data; // <date_value, quantity>
    };

    class MovementDividendManager : public finances::accounts::models::ModelManager<MovementDividend> {
      public:
        tl::expected<std::vector<MovementDividend>, finances::accounts::models::Error>
        all(finances::accounts::models::Id account_id);
    };
} // namespace finances::investments::models
