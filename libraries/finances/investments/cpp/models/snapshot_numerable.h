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

    // class SnapshotNumerableManager : public ModelManager<finances::investments::models::SnapshotNumerable> {
    //   public:
    //     using ModelManager<finances::investments::models::SnapshotNumerable>::ModelManager;

    //     ExpectedType<int, DatabaseError> create(utils::db::Id, utils::libpqxx::Date,
    //     finances::accounts::models::Amount,
    //                                             finances::accounts::models::Amount) {
    //         return tl::unexpected{NotImplemented{}};
    //     };
    // };

} // namespace utils::db
