#pragma once

#include "libraries/finances/accounts/cpp/models/snapshot.h"

namespace finances::investments::models {
    struct SnapshotNumerable {
        finances::accounts::models::Snapshot snapshot;

        utils::db::Id id;
        // FIXME: Implement NumerableAmount as a type. This pair is very error prone
        finances::accounts::models::Amount quantity;
        finances::accounts::models::Amount unit_value;
    };

} // namespace finances::investments::models

namespace utils::db {

    template <>
    template <>
    std::vector<finances::investments::models::SnapshotNumerable>
    utils::db::ModelManager<finances::investments::models::SnapshotNumerable>::_filter_by_fk<
        finances::accounts::models::Account>(pqxx::work&, const ModelData<finances::accounts::models::Account>::Id& id);

    template <>
    Id ModelManager<finances::investments::models::SnapshotNumerable>::_create(
        pqxx::work&, finances::investments::models::SnapshotNumerable&&);

} // namespace utils::db
