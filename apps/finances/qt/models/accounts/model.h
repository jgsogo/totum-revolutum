#pragma once

#include "libraries/finances/accounts/cpp/models/account.h"
#include "libraries/finances/accounts/cpp/models/account_holder.h"
#include "libraries/finances/accounts/cpp/models/snapshot.h"

struct AccountModel {
    decltype(finances::accounts::models::Account::id) id;

    finances::accounts::models::Account account;
    std::optional<finances::accounts::models::Snapshot> last_snapshot;
    std::vector<std::pair<finances::accounts::models::AccountHolder, finances::accounts::models::AccountHolderRole>>
        holders;
};

namespace utils::db {

    template <> std::vector<AccountModel> utils::db::Model<AccountModel>::get_all(pqxx::work&);

}
