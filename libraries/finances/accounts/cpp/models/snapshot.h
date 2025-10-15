#pragma once

#include <optional>
#include <string>

#include "libraries/utils/cpp/libpqxx/datatypes/date.h"
#include "libraries/utils/cpp/libpqxx/orm/manager.h"

#include "account.h"
#include "types/amount.h"

namespace finances::accounts::models {

    struct Snapshot {
        utils::db::Id id;
        std::pair<decltype(Account::id), decltype(Account::name)> account;
        utils::libpqxx::Date date_value;
        Amount amount;
    };

} // namespace finances::accounts::models

namespace utils::db {

    class SnapshotManager : public ModelManager<finances::accounts::models::Snapshot> {
      public:
        using ModelManager<finances::accounts::models::Snapshot>::ModelManager;

        ExpectedType<std::optional<finances::accounts::models::Snapshot>, DatabaseError>
        get_last_snapshot(const decltype(finances::accounts::models::Account::id)& account_id);

        ExpectedType<std::vector<std::optional<finances::accounts::models::Snapshot>>, DatabaseError>
        get_last_snapshots(const std::vector<decltype(finances::accounts::models::Account::id)>& account_ids);
    };

    template <>
    template <>
    std::vector<finances::accounts::models::Snapshot>
    utils::db::ModelManager<finances::accounts::models::Snapshot>::_filter_by_fk<finances::accounts::models::Account>(
        pqxx::work&, const ModelData<finances::accounts::models::Account>::Id& id);

    // auto r = manager.create(account.id, std::move(date), std::move(amount_amount.value()));

} // namespace utils::db
