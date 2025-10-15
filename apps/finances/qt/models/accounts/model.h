#pragma once

#include "libraries/finances/accounts/cpp/models/account.h"
#include "libraries/finances/accounts/cpp/models/account_holder.h"
#include "libraries/finances/accounts/cpp/models/snapshot.h"

struct AccountModel {
    utils::db::Id id;

    finances::accounts::models::Account account;
    std::optional<finances::accounts::models::Snapshot> last_snapshot;
    std::vector<finances::accounts::models::AccountHolderWithRoles> holders;
    std::string account_type_breadcrumb;
};

namespace utils::db {

    template <> ExpectedType<std::vector<AccountModel>, DatabaseError> ModelManager<AccountModel>::all();

} // namespace utils::db
