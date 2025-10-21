#pragma once

#include "libraries/finances/accounts/cpp/models/movement.h"
#include "snapshot_numerable.h"
namespace finances::investments::models {
    struct MovementDividend {
        finances::accounts::models::Movement movement;

        utils::db::Id id;
        utils::libpqxx::Date ex_dividend_date;
        finances::accounts::models::Money unit_value;

        std::optional<std::pair<decltype(decltype(SnapshotNumerable::snapshot)::date_value),
                                decltype(SnapshotNumerable::quantity)>>
            snapshot_data; // <date_value, quantity>
    };

} // namespace finances::investments::models

namespace utils::db {

    template <>
    template <>
    std::vector<finances::investments::models::MovementDividend>
    utils::db::ModelManager<finances::investments::models::MovementDividend>::_filter_by_fk<
        finances::accounts::models::Account>(pqxx::work&, const ModelData<finances::accounts::models::Account>::Id& id);

    template <>
    template <>
    std::vector<finances::investments::models::MovementDividend>
    utils::db::ModelManager<finances::investments::models::MovementDividend>::_filter_by_fk<
        finances::accounts::models::Transaction>(pqxx::work&,
                                                 const ModelData<finances::accounts::models::Transaction>::Id& id);
} // namespace utils::db
