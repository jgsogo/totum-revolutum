#pragma once

#include "libraries/finances/accounts/cpp/models/movement.h"
#include "snapshot_numerable.h"
namespace finances::investments::models {
    struct MovementDividend {
        finances::accounts::models::Movement movement;

        utils::db::Id id;
        utils::libpqxx::Date ex_dividend_date;
        finances::accounts::models::Amount unit_value;

        std::optional<std::pair<decltype(decltype(SnapshotNumerable::snapshot)::date_value),
                                decltype(SnapshotNumerable::quantity)>>
            snapshot_data; // <date_value, quantity>
    };

} // namespace finances::investments::models
