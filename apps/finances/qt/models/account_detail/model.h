#pragma once

#include "libraries/finances/accounts/cpp/models/account.h"
#include "libraries/finances/accounts/cpp/models/movement.h"
#include "libraries/finances/accounts/cpp/models/snapshot.h"

#include "libraries/utils/cpp/libpqxx/orm/manager.h"

struct AccountDetail {
    utils::db::Id id;

    std::vector<finances::accounts::models::Movement> movements;
    std::vector<finances::accounts::models::Snapshot> snapshots;
};

namespace utils::db {

    template <>
    ExpectedType<AccountDetail, DatabaseError, ErrorNotFound, ErrorMultipleFound>
    ModelManager<AccountDetail>::get(const ModelData<AccountDetail>::Id& id);

} // namespace utils::db
