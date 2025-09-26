#pragma once

#include <optional>
#include <string>

#include "libraries/utils/cpp/libpqxx/datatypes/date.h"

#include "account.h"
#include "types/amount.h"
#include "types/id.h"

namespace finances::accounts::models {

    class SnapshotManager;

    struct Snapshot {
        using Manager = SnapshotManager;

        Id id;
        decltype(AccountType::id) account_id;
        utils::libpqxx::Date date_value;
        Amount amount;
    };

    class SnapshotManager : public ModelManager<Snapshot> {
      public:
        tl::expected<std::optional<Snapshot>, Error> get_last_snapshot(decltype(Account::id) account_id) const;

        tl::expected<std::vector<std::optional<Snapshot>>, Error>
        get_last_snapshots(const std::vector<decltype(Account::id)>& account_ids) const;

        tl::expected<std::vector<Snapshot>, Error> all(Id account_id);
    };

} // namespace finances::accounts::models
