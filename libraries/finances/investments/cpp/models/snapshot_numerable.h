#pragma once

#include "libraries/finances/accounts/cpp/models/snapshot.h"

namespace finances::investments::models {
    class SnapshotNumerableManager;

    struct SnapshotNumerable : finances::accounts::models::Snapshot {
        using Manager = SnapshotNumerableManager;

        finances::accounts::models::Id id;
        finances::accounts::models::Amount quantity;
        finances::accounts::models::Amount unit_value;
    };

    class SnapshotNumerableManager : public finances::accounts::models::ModelManager<SnapshotNumerable> {
      public:
        tl::expected<std::vector<SnapshotNumerable>, finances::accounts::models::Error>
        all(finances::accounts::models::Id account_id);
        tl::expected<void, finances::accounts::models::Error> create(finances::accounts::models::Id account_id,
                                                                     utils::libpqxx::Date&& date_value,
                                                                     finances::accounts::models::Amount&& quantity,
                                                                     finances::accounts::models::Amount&& unit_value);
    };
} // namespace finances::investments::models
