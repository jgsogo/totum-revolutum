#pragma once

#include "libraries/finances/accounts/cpp/models/movement.h"

namespace finances::investments::models {
    struct MovementNumerable {
        finances::accounts::models::Movement movement;

        utils::db::Id id;
        finances::accounts::models::Amount quantity;
        finances::accounts::models::Amount unit_value;
    };

} // namespace finances::investments::models

namespace utils::db {

    template <>
    template <>
    std::vector<finances::investments::models::MovementNumerable>
    utils::db::ModelManager<finances::investments::models::MovementNumerable>::_filter_by_fk<
        finances::accounts::models::Account>(pqxx::work&, const ModelData<finances::accounts::models::Account>::Id& id);

    template <>
    template <>
    std::vector<finances::investments::models::MovementNumerable>
    utils::db::ModelManager<finances::investments::models::MovementNumerable>::_filter_by_fk<
        finances::accounts::models::Transaction>(pqxx::work&,
                                                 const ModelData<finances::accounts::models::Transaction>::Id& id);

} // namespace utils::db
