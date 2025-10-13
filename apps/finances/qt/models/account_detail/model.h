#pragma once

#include "libraries/finances/accounts/cpp/models/account.h"
#include "libraries/finances/accounts/cpp/models/movement.h"
#include "libraries/finances/accounts/cpp/models/snapshot.h"

#include "libraries/utils/cpp/libpqxx/orm/manager.h"

struct AccountDetail {
    // using Id = decltype(finances::accounts::models::Account::id);
    // const Id id;

    using Id = finances::accounts::models::Account;
    const Id id;

    std::vector<finances::accounts::models::Movement> movements;
    std::vector<finances::accounts::models::Snapshot> snapshots;
};

namespace utils::db {

    template <>
    ExpectedType<AccountDetail, DatabaseError, ErrorNotFound, ErrorMultipleFound>
    ModelManager<AccountDetail>::get(const ModelData<AccountDetail>::Id& id);

} // namespace utils::db
