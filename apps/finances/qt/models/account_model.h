#pragma once

#include "libraries/finances/accounts/cpp/models/account.h"
#include "libraries/finances/accounts/cpp/models/account_holder.h"
#include "libraries/finances/accounts/cpp/models/snapshot.h"
#include "libraries/finances/investments/cpp/models/snapshot_numerable.h"

/// A model wrapping finances::accounts::movel::Account with some additional data
struct AccountModel {
    utils::db::Id id;

    finances::accounts::models::Account account;
    std::optional<std::variant<finances::accounts::models::Snapshot, finances::investments::models::SnapshotNumerable>>
        last_snapshot;
    std::vector<finances::accounts::models::AccountHolderWithRoles> holders;
    std::string account_type_breadcrumb;

    std::optional<std::reference_wrapper<const utils::libpqxx::Date>> last_snapshot_date() const;
    std::optional<std::reference_wrapper<const finances::accounts::models::Money>> last_snapshot_amount() const;
};

namespace utils::db {

    template <> ExpectedType<std::vector<AccountModel>, DatabaseError> ModelManager<AccountModel>::all();

    template <>
    ExpectedType<AccountModel, DatabaseError, ErrorNotFound, ErrorMultipleFound>
    ModelManager<AccountModel>::get(const ModelData<AccountModel>::Id&);

} // namespace utils::db
