#pragma once

#include <optional>
#include <string>

#include "libraries/utils/cpp/libpqxx/datatypes/date.h"
#include "libraries/utils/cpp/libpqxx/orm/manager.h"

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

        tl::expected<void, Error> create(Id account_id, utils::libpqxx::Date&& date_value, Amount&& amount);
    };

} // namespace finances::accounts::models

namespace utils::db {

    class SnapshotManager : public ModelManager<finances::accounts::models::Snapshot> {
      public:
        using ModelManager<finances::accounts::models::Snapshot>::ModelManager;

        ExpectedType<std::optional<finances::accounts::models::Snapshot>, DatabaseError>
        get_last_snapshot(const decltype(finances::accounts::models::Account::id)& account_id);
    };

    template <>
    template <>
    std::vector<finances::accounts::models::Snapshot>
    utils::db::ModelManager<finances::accounts::models::Snapshot>::_filter_by_fk<finances::accounts::models::Account>(
        pqxx::work&, const ModelData<finances::accounts::models::Account>::Id& id);

} // namespace utils::db
